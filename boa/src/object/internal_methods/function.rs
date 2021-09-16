use crate::{
    builtins::function::{
        create_unmapped_arguments_object, ClosureFunction, Function, NativeFunction,
    },
    environment::{
        environment_record_trait::EnvironmentRecordTrait,
        function_environment_record::{BindingStatus, FunctionEnvironmentRecord},
        lexical_environment::Environment,
    },
    exec::{Executable, InterpreterState},
    object::{JsObject, Object, PROTOTYPE},
    syntax::ast::node::RcStatementList,
    Context, JsResult, JsValue,
};

use super::{InternalObjectMethods, ORDINARY_INTERNAL_METHODS};

/// Definitions of the internal object methods for function objects.
///
/// More information:
///  - [ECMAScript reference][spec]
///
/// [spec]: https://tc39.es/ecma262/#sec-ecmascript-function-objects
pub(crate) static FUNCTION_INTERNAL_METHODS: InternalObjectMethods = InternalObjectMethods {
    __call__: Some(function_call),
    __construct__: None,
    ..ORDINARY_INTERNAL_METHODS
};

pub(crate) static CONSTRUCTOR_INTERNAL_METHODS: InternalObjectMethods = InternalObjectMethods {
    __call__: Some(function_call),
    __construct__: Some(function_construct),
    ..ORDINARY_INTERNAL_METHODS
};

/// The body of a JavaScript function.
///
/// This is needed for the call method since we cannot mutate the function itobj since we
/// already borrow it so we get the function body clone it then drop the borrow and run the body
enum FunctionBody {
    BuiltInFunction(NativeFunction),
    BuiltInConstructor(NativeFunction),
    Closure(Box<dyn ClosureFunction>),
    Ordinary(RcStatementList),
}

/// Call this object.
///
/// # Panics
///
/// Panics if the object is currently mutably borrowed.
// <https://tc39.es/ecma262/#sec-prepareforordinarycall>
// <https://tc39.es/ecma262/#sec-ecmascript-function-objects-call-thisargument-argumentslist>
#[track_caller]
#[inline]
fn function_call(
    obj: &JsObject,
    this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    call_construct(obj, this, args, context, false)
}

/// Construct an instance of this object with the specified arguments.
///
/// # Panics
///
/// Panics if the object is currently mutably borrowed.
// <https://tc39.es/ecma262/#sec-ecmascript-function-objects-construct-argumentslist-newtarget>
#[track_caller]
#[inline]
fn function_construct(
    obj: &JsObject,
    args: &[JsValue],
    new_target: &JsValue,
    context: &mut Context,
) -> JsResult<JsValue> {
    call_construct(obj, new_target, args, context, true)
}

/// Internal implementation of [`call`](#method.call) and [`construct`](#method.construct).
///
/// # Panics
///
/// Panics if the object is currently mutably borrowed.
///
/// <https://tc39.es/ecma262/#sec-prepareforordinarycall>
/// <https://tc39.es/ecma262/#sec-ordinarycallbindthis>
/// <https://tc39.es/ecma262/#sec-runtime-semantics-evaluatebody>
/// <https://tc39.es/ecma262/#sec-ordinarycallevaluatebody>
#[track_caller]
fn call_construct(
    obj: &JsObject,
    this_target: &JsValue,
    args: &[JsValue],
    context: &mut Context,
    construct: bool,
) -> JsResult<JsValue> {
    let this_function_object = obj.clone();
    let mut has_parameter_expressions = false;

    let body = if let Some(function) = obj.borrow().as_function() {
        if construct && !function.is_constructor() {
            let name = obj
                .__get__(&"name".into(), obj.clone().into(), context)?
                .display()
                .to_string();
            return context.throw_type_error(format!("{} is not a constructor", name));
        } else {
            match function {
                Function::Native {
                    function,
                    constructor: constructable,
                } => {
                    if *constructable || construct {
                        FunctionBody::BuiltInConstructor(function.0)
                    } else {
                        FunctionBody::BuiltInFunction(function.0)
                    }
                }
                Function::Closure { function, .. } => FunctionBody::Closure(function.clone()),
                Function::Ordinary {
                    body,
                    params,
                    environment,
                    flags,
                } => {
                    let this = if construct {
                        // If the prototype of the constructor is not an object, then use the default object
                        // prototype as prototype for the new object
                        // see <https://tc39.es/ecma262/#sec-ordinarycreatefromconstructor>
                        // see <https://tc39.es/ecma262/#sec-getprototypefromconstructor>
                        let proto = this_target.as_object().unwrap().__get__(
                            &PROTOTYPE.into(),
                            this_target.clone(),
                            context,
                        )?;
                        let proto = if proto.is_object() {
                            proto
                        } else {
                            context
                                .standard_objects()
                                .object_object()
                                .prototype()
                                .into()
                        };
                        JsValue::new(Object::create(proto))
                    } else {
                        this_target.clone()
                    };

                    // Create a new Function environment whose parent is set to the scope of the function declaration (obj.environment)
                    // <https://tc39.es/ecma262/#sec-prepareforordinarycall>
                    let local_env = FunctionEnvironmentRecord::new(
                        this_function_object.clone(),
                        if construct || !flags.is_lexical_this_mode() {
                            Some(this.clone())
                        } else {
                            None
                        },
                        Some(environment.clone()),
                        // Arrow functions do not have a this binding https://tc39.es/ecma262/#sec-function-environment-records
                        if flags.is_lexical_this_mode() {
                            BindingStatus::Lexical
                        } else {
                            BindingStatus::Uninitialized
                        },
                        JsValue::undefined(),
                        context,
                    )?;

                    let mut arguments_in_parameter_names = false;

                    for param in params.iter() {
                        has_parameter_expressions =
                            has_parameter_expressions || param.init().is_some();
                        arguments_in_parameter_names =
                            arguments_in_parameter_names || param.name() == "arguments";
                    }

                    // An arguments object is added when all of the following conditions are met
                    // - If not in an arrow function (10.2.11.16)
                    // - If the parameter list does not contain `arguments` (10.2.11.17)
                    // - If there are default parameters or if lexical names and function names do not contain `arguments` (10.2.11.18)
                    //
                    // https://tc39.es/ecma262/#sec-functiondeclarationinstantiation
                    if !flags.is_lexical_this_mode()
                        && !arguments_in_parameter_names
                        && (has_parameter_expressions
                            || (!body.lexically_declared_names().contains("arguments")
                                && !body.function_declared_names().contains("arguments")))
                    {
                        // Add arguments object
                        let arguments_obj = create_unmapped_arguments_object(args, context)?;
                        local_env.create_mutable_binding("arguments", false, true, context)?;
                        local_env.initialize_binding("arguments", arguments_obj, context)?;
                    }

                    // Turn local_env into Environment so it can be cloned
                    let local_env: Environment = local_env.into();

                    // Push the environment first so that it will be used by default parameters
                    context.push_environment(local_env.clone());

                    // Add argument bindings to the function environment
                    for (i, param) in params.iter().enumerate() {
                        // Rest Parameters
                        if param.is_rest_param() {
                            function.add_rest_param(param, i, args, context, &local_env);
                            break;
                        }

                        let value = match args.get(i).cloned() {
                            None | Some(JsValue::Undefined) => param
                                .init()
                                .map(|init| init.run(context).ok())
                                .flatten()
                                .unwrap_or_default(),
                            Some(value) => value,
                        };

                        function.add_arguments_to_environment(param, value, &local_env, context);
                    }

                    if has_parameter_expressions {
                        // Create a second environment when default parameter expressions are used
                        // This prevents variables declared in the function body from being
                        // used in default parameter initializers.
                        // https://tc39.es/ecma262/#sec-functiondeclarationinstantiation
                        let second_env = FunctionEnvironmentRecord::new(
                            this_function_object,
                            if construct || !flags.is_lexical_this_mode() {
                                Some(this)
                            } else {
                                None
                            },
                            Some(local_env),
                            // Arrow functions do not have a this binding https://tc39.es/ecma262/#sec-function-environment-records
                            if flags.is_lexical_this_mode() {
                                BindingStatus::Lexical
                            } else {
                                BindingStatus::Uninitialized
                            },
                            JsValue::undefined(),
                            context,
                        )?;
                        context.push_environment(second_env);
                    }

                    FunctionBody::Ordinary(body.clone())
                }
            }
        }
    } else {
        return context.throw_type_error("not a function");
    };

    match body {
        FunctionBody::BuiltInConstructor(function) if construct => {
            function(this_target, args, context)
        }
        FunctionBody::BuiltInConstructor(function) => {
            function(&JsValue::undefined(), args, context)
        }
        FunctionBody::BuiltInFunction(function) => function(this_target, args, context),
        FunctionBody::Closure(function) => (function)(this_target, args, context),
        FunctionBody::Ordinary(body) => {
            let result = body.run(context);
            let this = context.get_this_binding();

            if has_parameter_expressions {
                context.pop_environment();
            }
            context.pop_environment();

            if construct {
                // https://tc39.es/ecma262/#sec-ecmascript-function-objects-construct-argumentslist-newtarget
                // 12. If result.[[Type]] is return, then
                if context.executor().get_current_state() == &InterpreterState::Return {
                    // a. If Type(result.[[Value]]) is Object, return NormalCompletion(result.[[Value]]).
                    if let Ok(v) = &result {
                        if v.is_object() {
                            return result;
                        }
                    }
                }

                // 13. Else, ReturnIfAbrupt(result).
                result?;

                // 14. Return ? constructorEnv.GetThisBinding().
                this
            } else if context.executor().get_current_state() == &InterpreterState::Return {
                result
            } else {
                result?;
                Ok(JsValue::undefined())
            }
        }
    }
}
