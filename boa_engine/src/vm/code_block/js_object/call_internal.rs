use crate::{
    builtins::{
        async_generator::{AsyncGenerator, AsyncGeneratorState},
        function::{
            arguments::Arguments, Function, ThisMode,
        },
        generator::{Generator, GeneratorContext, GeneratorState},
    },
    error::JsNativeError,
    object::{
        JsObject, ObjectData, 
    },
    vm::call_frame::GeneratorResumeKind,
    vm::{call_frame::FinallyReturn, CallFrame}, 
    Context, JsResult, JsValue,
};
use boa_gc::{Gc, GcCell};
use std::collections::VecDeque;


impl JsObject {
    pub(crate) fn call_internal(
        &self,
        this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let this_function_object = self.clone();

        if !self.is_callable() {
            return Err(JsNativeError::typ()
                .with_message("not a callable function")
                .into());
        }

        let object = self.borrow();
        let function_object = object.as_function().expect("not a function");

        match function_object {
            Function::Native {
                function,
                constructor,
            } => {
                let function = *function;
                let constructor = *constructor;
                drop(object);

                if constructor.is_some() {
                    function(&JsValue::undefined(), args, context)
                } else {
                    function(this, args, context)
                }
            }
            Function::Closure {
                function, captures, ..
            } => {
                let function = function.clone();
                let captures = captures.clone();
                drop(object);

                (function)(this, args, captures, context)
            }
            Function::Ordinary {
                code, environments, ..
            } => {
                let code = code.clone();
                let mut environments = environments.clone();
                drop(object);

                if code.is_class_constructor {
                    return Err(JsNativeError::typ()
                        .with_message("Class constructor cannot be invoked without 'new'")
                        .into());
                }

                let environments_len = environments.len();
                std::mem::swap(&mut environments, &mut context.realm.environments);

                let lexical_this_mode = code.this_mode == ThisMode::Lexical;

                let this = if lexical_this_mode {
                    None
                } else if code.strict {
                    Some(this.clone())
                } else if this.is_null_or_undefined() {
                    Some(context.global_object().clone().into())
                } else {
                    Some(
                        this.to_object(context)
                            .expect("conversion cannot fail")
                            .into(),
                    )
                };

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
                    this,
                    self.clone(),
                    None,
                    lexical_this_mode,
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
                context.vm.pop_frame().expect("must have frame");

                std::mem::swap(&mut environments, &mut context.realm.environments);
                environments.truncate(environments_len);

                let (result, _) = result?;
                Ok(result)
            }
            Function::Async {
                code,
                environments,
                promise_capability,
                ..
            } => {
                let code = code.clone();
                let mut environments = environments.clone();
                let promise = promise_capability.promise().clone();
                drop(object);

                let environments_len = environments.len();
                std::mem::swap(&mut environments, &mut context.realm.environments);

                let lexical_this_mode = code.this_mode == ThisMode::Lexical;

                let this = if lexical_this_mode {
                    None
                } else if code.strict {
                    Some(this.clone())
                } else if this.is_null_or_undefined() {
                    Some(context.global_object().clone().into())
                } else {
                    Some(
                        this.to_object(context)
                            .expect("conversion cannot fail")
                            .into(),
                    )
                };

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
                    this,
                    self.clone(),
                    None,
                    lexical_this_mode,
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

                let _result = context.run();
                context.vm.pop_frame().expect("must have frame");

                std::mem::swap(&mut environments, &mut context.realm.environments);
                environments.truncate(environments_len);

                Ok(promise.into())
            }
            Function::Generator {
                code, environments, ..
            } => {
                let code = code.clone();
                let mut environments = environments.clone();
                drop(object);

                std::mem::swap(&mut environments, &mut context.realm.environments);

                let lexical_this_mode = code.this_mode == ThisMode::Lexical;

                let this = if lexical_this_mode {
                    None
                } else if code.strict {
                    Some(this.clone())
                } else if this.is_null_or_undefined() {
                    Some(context.global_object().clone().into())
                } else {
                    Some(
                        this.to_object(context)
                            .expect("conversion cannot fail")
                            .into(),
                    )
                };

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
                    this,
                    self.clone(),
                    None,
                    lexical_this_mode,
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
                let mut args = if code.params.as_ref().len() > args.len() {
                    let mut v = args.to_vec();
                    v.extend(vec![
                        JsValue::Undefined;
                        code.params.as_ref().len() - args.len()
                    ]);
                    v
                } else {
                    args.to_vec()
                };
                args.reverse();

                let param_count = code.params.as_ref().len();

                let call_frame = CallFrame {
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
                };
                let mut stack = args;

                std::mem::swap(&mut context.vm.stack, &mut stack);
                context.vm.push_frame(call_frame);

                let init_result = context.run();

                let call_frame = context.vm.pop_frame().expect("frame must exist");
                std::mem::swap(&mut environments, &mut context.realm.environments);
                std::mem::swap(&mut context.vm.stack, &mut stack);

                let prototype = this_function_object
                    .get("prototype", context)
                    .expect("GeneratorFunction must have a prototype property")
                    .as_object()
                    .map_or_else(
                        || context.intrinsics().constructors().generator().prototype(),
                        Clone::clone,
                    );

                let generator = Self::from_proto_and_data(
                    prototype,
                    ObjectData::generator(Generator {
                        state: GeneratorState::SuspendedStart,
                        context: Some(Gc::new(GcCell::new(GeneratorContext {
                            environments,
                            call_frame,
                            stack,
                        }))),
                    }),
                );

                init_result?;

                Ok(generator.into())
            }
            Function::AsyncGenerator {
                code, environments, ..
            } => {
                let code = code.clone();
                let mut environments = environments.clone();
                drop(object);

                std::mem::swap(&mut environments, &mut context.realm.environments);

                let lexical_this_mode = code.this_mode == ThisMode::Lexical;

                let this = if lexical_this_mode {
                    None
                } else if code.strict {
                    Some(this.clone())
                } else if this.is_null_or_undefined() {
                    Some(context.global_object().clone().into())
                } else {
                    Some(
                        this.to_object(context)
                            .expect("conversion cannot fail")
                            .into(),
                    )
                };

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
                    this,
                    self.clone(),
                    None,
                    lexical_this_mode,
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
                let mut args = if code.params.as_ref().len() > args.len() {
                    let mut v = args.to_vec();
                    v.extend(vec![
                        JsValue::Undefined;
                        code.params.as_ref().len() - args.len()
                    ]);
                    v
                } else {
                    args.to_vec()
                };
                args.reverse();

                let param_count = code.params.as_ref().len();

                let call_frame = CallFrame {
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
                };
                let mut stack = args;

                std::mem::swap(&mut context.vm.stack, &mut stack);
                context.vm.push_frame(call_frame);

                let init_result = context.run();

                let call_frame = context.vm.pop_frame().expect("frame must exist");
                std::mem::swap(&mut environments, &mut context.realm.environments);
                std::mem::swap(&mut context.vm.stack, &mut stack);

                let prototype = this_function_object
                    .get("prototype", context)
                    .expect("AsyncGeneratorFunction must have a prototype property")
                    .as_object()
                    .map_or_else(
                        || {
                            context
                                .intrinsics()
                                .constructors()
                                .async_generator()
                                .prototype()
                        },
                        Clone::clone,
                    );

                let generator = Self::from_proto_and_data(
                    prototype,
                    ObjectData::async_generator(AsyncGenerator {
                        state: AsyncGeneratorState::SuspendedStart,
                        context: Some(Gc::new(GcCell::new(GeneratorContext {
                            environments,
                            call_frame,
                            stack,
                        }))),
                        queue: VecDeque::new(),
                    }),
                );

                {
                    let mut generator_mut = generator.borrow_mut();
                    let gen = generator_mut
                        .as_async_generator_mut()
                        .expect("must be object here");
                    let mut gen_context = gen.context.as_ref().expect("must exist").borrow_mut();
                    gen_context.call_frame.async_generator = Some(generator.clone());
                }

                init_result?;

                Ok(generator.into())
            }
        }
    }
}