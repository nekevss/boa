use crate::{
    builtins::{
        function::{
            arguments::Arguments, Function,
        },
    },
    context::intrinsics::StandardConstructors,
    error::JsNativeError,
    object::{
        internal_methods::get_prototype_from_constructor, JsObject, ObjectData,
    },
    vm::call_frame::GeneratorResumeKind,
    vm::{call_frame::FinallyReturn, CallFrame}, 
    Context, JsResult, JsValue,
};

impl JsObject {
    pub(crate) fn construct_internal(
        &self,
        args: &[JsValue],
        this_target: &JsValue,
        context: &mut Context,
    ) -> JsResult<Self> {
        let this_function_object = self.clone();

        let create_this = |context| {
            let prototype =
                get_prototype_from_constructor(this_target, StandardConstructors::object, context)?;
            Ok(Self::from_proto_and_data(prototype, ObjectData::ordinary()))
        };

        if !self.is_constructor() {
            return Err(JsNativeError::typ()
                .with_message("not a constructor function")
                .into());
        }

        let object = self.borrow();
        let function_object = object.as_function().expect("not a function");

        match function_object {
            Function::Native {
                function,
                constructor,
                ..
            } => {
                let function = *function;
                let constructor = *constructor;
                drop(object);

                match function(this_target, args, context)? {
                    JsValue::Object(ref o) => Ok(o.clone()),
                    val => {
                        if constructor.expect("hmm").is_base() || val.is_undefined() {
                            create_this(context)
                        } else {
                            Err(JsNativeError::typ()
                                .with_message(
                                    "Derived constructor can only return an Object or undefined",
                                )
                                .into())
                        }
                    }
                }
            }
            Function::Closure {
                function,
                captures,
                constructor,
                ..
            } => {
                let function = function.clone();
                let captures = captures.clone();
                let constructor = *constructor;
                drop(object);

                match (function)(this_target, args, captures, context)? {
                    JsValue::Object(ref o) => Ok(o.clone()),
                    val => {
                        if constructor.expect("hmma").is_base() || val.is_undefined() {
                            create_this(context)
                        } else {
                            Err(JsNativeError::typ()
                                .with_message(
                                    "Derived constructor can only return an Object or undefined",
                                )
                                .into())
                        }
                    }
                }
            }
            Function::Ordinary {
                code,
                environments,
                constructor_kind,
                ..
            } => {
                let code = code.clone();
                let mut environments = environments.clone();
                let constructor_kind = *constructor_kind;
                drop(object);

                let environments_len = environments.len();
                std::mem::swap(&mut environments, &mut context.realm.environments);

                let this = if constructor_kind.is_base() {
                    // If the prototype of the constructor is not an object, then use the default object
                    // prototype as prototype for the new object
                    // see <https://tc39.es/ecma262/#sec-ordinarycreatefromconstructor>
                    // see <https://tc39.es/ecma262/#sec-getprototypefromconstructor>
                    let prototype = get_prototype_from_constructor(
                        this_target,
                        StandardConstructors::object,
                        context,
                    )?;
                    let this = Self::from_proto_and_data(prototype, ObjectData::ordinary());

                    Self::initialize_instance_elements(&this, self, context)?;

                    Some(this)
                } else {
                    None
                };

                let new_target = this_target.as_object().expect("must be object");

                let compile_time_environment_index = usize::from(code.params.has_expressions());

                if code.has_binding_identifier {
                    let index = context.realm.environments.push_declarative(
                        1,
                        code.compile_environments[compile_time_environment_index + 1].clone(),
                    );
                    context
                        .realm
                        .environments
                        .put_value(index, 0, self.clone().into());
                }

                context.realm.environments.push_function(
                    code.num_bindings,
                    code.compile_environments[compile_time_environment_index].clone(),
                    this.clone().map(Into::into),
                    self.clone(),
                    Some(new_target.clone()),
                    false,
                );

                if let Some(binding) = code.arguments_binding {
                    let arguments_obj = if code.strict || !code.params.is_simple() {
                        Arguments::create_unmapped_arguments_object(args, context)
                    } else {
                        let env = context.realm.environments.current();
                        Arguments::create_mapped_arguments_object(
                            &this_function_object,
                            &code.params,
                            args,
                            &env,
                            context,
                        )
                    };
                    context.realm.environments.put_value(
                        binding.environment_index(),
                        binding.binding_index(),
                        arguments_obj.into(),
                    );
                }

                let arg_count = args.len();

                // Push function arguments to the stack.
                let args = if code.params.as_ref().len() > args.len() {
                    let mut v = args.to_vec();
                    v.extend(vec![
                        JsValue::Undefined;
                        code.params.as_ref().len() - args.len()
                    ]);
                    v
                } else {
                    args.to_vec()
                };

                for arg in args.iter().rev() {
                    context.vm.push(arg);
                }

                let param_count = code.params.as_ref().len();
                let has_binding_identifier = code.has_binding_identifier;

                context.vm.push_frame(CallFrame {
                    code,
                    pc: 0,
                    catch: Vec::new(),
                    finally_return: FinallyReturn::None,
                    finally_jump: Vec::new(),
                    pop_on_return: 0,
                    loop_env_stack: Vec::from([0]),
                    try_env_stack: Vec::from([crate::vm::TryStackEntry {
                        num_env: 0,
                        num_loop_stack_entries: 0,
                    }]),
                    param_count,
                    arg_count,
                    generator_resume_kind: GeneratorResumeKind::Normal,
                    thrown: false,
                    async_generator: None,
                });

                let result = context.run();

                context.vm.pop_frame();

                std::mem::swap(&mut environments, &mut context.realm.environments);

                let environment = if has_binding_identifier {
                    environments.truncate(environments_len + 2);
                    let environment = environments.pop();
                    environments.pop();
                    environment
                } else {
                    environments.truncate(environments_len + 1);
                    environments.pop()
                };

                let (result, _) = result?;

                if let Some(result) = result.as_object() {
                    Ok(result.clone())
                } else if let Some(this) = this {
                    Ok(this)
                } else if !result.is_undefined() {
                    Err(JsNativeError::typ()
                        .with_message("Function constructor must not return non-object")
                        .into())
                } else {
                    let function_env = environment
                        .slots()
                        .expect("must be function environment")
                        .as_function_slots()
                        .expect("must be function environment");
                    Ok(function_env
                        .borrow()
                        .get_this_binding()?
                        .as_object()
                        .expect("this binding must be object")
                        .clone())
                }
            }
            Function::Generator { .. }
            | Function::Async { .. }
            | Function::AsyncGenerator { .. } => {
                unreachable!("not a constructor")
            }
        }
    }
}