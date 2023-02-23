use crate::{
    vm::{opcode::Operation, CompletionType},
    Context,
};

/// `FinallyStart` implements the Opcode Operation for `Opcode::FinallyStart`
///
/// Operation:
///  - Start of a finally block.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FinallyStart;

impl Operation for FinallyStart {
    const NAME: &'static str = "FinallyStart";
    const INSTRUCTION: &'static str = "INST - FinallyStart";

    fn execute(context: &mut Context<'_>) -> CompletionType {
        let exit = context.vm.read::<u32>();

        let finally_env = context
            .vm
            .frame_mut()
            .env_stack
            .last_mut()
            .expect("EnvStackEntries must exist");

        finally_env.set_exit_address(exit);
        CompletionType::Normal
    }
}

/// `FinallyEnd` implements the Opcode Operation for `Opcode::FinallyEnd`
///
/// Operation:
///  - End of a finally block.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FinallyEnd;

impl Operation for FinallyEnd {
    const NAME: &'static str = "FinallyEnd";
    const INSTRUCTION: &'static str = "INST - FinallyEnd";

    fn execute(context: &mut Context<'_>) -> CompletionType {
        let finally_candidates = context.vm.frame().env_stack.iter().filter(|env| {
            env.is_finally_env() && context.vm.frame().pc < (env.start_address() as usize)
        });

        let next_finally = match finally_candidates.last() {
            Some(env) => env.start_address(),
            _ => u32::MAX,
        };

        let mut envs_to_pop = 0;
        match context.vm.frame().abrupt_completion {
            Some(record) if next_finally < record.target() => {
                context.vm.frame_mut().pc = next_finally as usize;

                while let Some(env_entry) = context.vm.frame().env_stack.last() {
                    if next_finally <= env_entry.exit_address() {
                        break;
                    }

                    envs_to_pop += env_entry.env_num();
                    context.vm.frame_mut().env_stack.pop();
                }

                let env_truncation_len =
                    context.realm.environments.len().saturating_sub(envs_to_pop);
                context.realm.environments.truncate(env_truncation_len);
            }
            Some(record)
                if record.is_break() && context.vm.frame().pc < record.target() as usize =>
            {
                // handle the continuation of an abrupt break.
                context.vm.frame_mut().pc = record.target() as usize;
                while let Some(env_entry) = context.vm.frame().env_stack.last() {
                    if record.target() == env_entry.exit_address() {
                        break;
                    }

                    envs_to_pop += env_entry.env_num();
                    context.vm.frame_mut().env_stack.pop();
                }

                context.vm.frame_mut().abrupt_completion = None;

                let env_truncation_len =
                    context.realm.environments.len().saturating_sub(envs_to_pop);
                context.realm.environments.truncate(env_truncation_len);
            }
            Some(record)
                if record.is_continue() && context.vm.frame().pc > record.target() as usize =>
            {
                // Handle the continuation of an abrupt continue
                context.vm.frame_mut().pc = record.target() as usize;
                while let Some(env_entry) = context.vm.frame().env_stack.last() {
                    if env_entry.start_address() == record.target() {
                        break;
                    }
                    envs_to_pop += env_entry.env_num();
                    context.vm.frame_mut().env_stack.pop();
                }

                context.vm.frame_mut().abrupt_completion = None;
                let env_truncation_len =
                    context.realm.environments.len().saturating_sub(envs_to_pop);
                context.realm.environments.truncate(env_truncation_len);
            }
            Some(record)
                if record.is_throw_with_target()
                    && context.vm.frame().pc < record.target() as usize =>
            {
                context.vm.frame_mut().pc = record.target() as usize;
                while let Some(env_entry) = context.vm.frame_mut().env_stack.pop() {
                    envs_to_pop += env_entry.env_num();
                    if env_entry.start_address() == record.target() {
                        break;
                    }
                }
                context.vm.frame_mut().abrupt_completion = None;
                let env_truncation_len =
                    context.realm.environments.len().saturating_sub(envs_to_pop);
                context.realm.environments.truncate(env_truncation_len);
            }
            Some(record) if !record.is_throw_with_target() => {
                let current_stack = context
                    .vm
                    .frame_mut()
                    .env_stack
                    .pop()
                    .expect("Popping current finally stack.");

                let env_truncation_len = context
                    .realm
                    .environments
                    .len()
                    .saturating_sub(current_stack.env_num());
                context.realm.environments.truncate(env_truncation_len);

                return CompletionType::Throw;
            }
            Some(record) if record.is_return() => {
                // TODO: Implement logic for return completions (Should probably function similar to throw)
                return CompletionType::Return;
            }
            _ => {
                context.vm.frame_mut().env_stack.pop();
            }
        }

        CompletionType::Normal
    }
}