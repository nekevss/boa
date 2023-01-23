//! `CallFrame`
//!
//! This module will provides everything needed to implement the `CallFrame`

use crate::{object::JsObject, vm::CodeBlock};
use boa_gc::{Finalize, Gc, Trace};

mod abrupt_record;
mod env_stack;

pub(crate) use abrupt_record::AbruptCompletionRecord;
pub(crate) use env_stack::EnvStackEntry;

/// A `CallFrame` holds the state of a function call.
#[derive(Clone, Debug, Finalize, Trace)]
pub struct CallFrame {
    pub(crate) code_block: Gc<CodeBlock>,
    pub(crate) pc: usize,
    #[unsafe_ignore_trace]
    pub(crate) try_catch: Vec<TryAddresses>,
    #[unsafe_ignore_trace]
    pub(crate) finally_return: FinallyReturn,
    #[unsafe_ignore_trace]
    pub(crate) abrupt_completion: Option<AbruptCompletionRecord>,
    pub(crate) finally_jump: Vec<Option<u32>>,
    pub(crate) pop_on_return: usize,

    // Tracks the number of environments in the current loop block.
    // On abrupt returns this is used to decide how many environments need to be pop'ed.
    #[unsafe_ignore_trace]
    pub(crate) env_stack: Vec<EnvStackEntry>,
    pub(crate) param_count: usize,
    pub(crate) arg_count: usize,
    #[unsafe_ignore_trace]
    pub(crate) generator_resume_kind: GeneratorResumeKind,

    // Indicate that the last try block has thrown an exception.
    pub(crate) thrown: bool,

    // When an async generator is resumed, the generator object is needed
    // to fulfill the steps 4.e-j in [AsyncGeneratorStart](https://tc39.es/ecma262/#sec-asyncgeneratorstart).
    pub(crate) async_generator: Option<JsObject>,
}

/// ---- `CallFrame` creation methods ----
impl CallFrame {
    /// Creates a new `CallFrame` with the provided `CodeBlock`.
    pub(crate) fn new(code_block: Gc<CodeBlock>) -> Self {
        Self {
            code_block,
            pc: 0,
            try_catch: Vec::new(),
            finally_return: FinallyReturn::None,
            finally_jump: Vec::new(),
            pop_on_return: 0,
            env_stack: Vec::from([EnvStackEntry::default().with_initial_env_num(0)]),
            abrupt_completion: None,
            param_count: 0,
            arg_count: 0,
            generator_resume_kind: GeneratorResumeKind::Normal,
            thrown: false,
            async_generator: None,
        }
    }

    /// Updates a `CallFrame`'s `param_count` field with the value provided.
    pub(crate) fn with_param_count(mut self, count: usize) -> Self {
        self.param_count = count;
        self
    }

    /// Updates a `CallFrame`'s `arg_count` field with the value provided.
    pub(crate) fn with_arg_count(mut self, count: usize) -> Self {
        self.arg_count = count;
        self
    }
}

/// ---- `CallFrame` stack methods ----
impl CallFrame {
    /// Tracks that one environment has been pushed in the current loop block.
    pub(crate) fn inc_frame_env_stack(&mut self) {
        self.env_stack
            .last_mut()
            .expect("environment stack entry must exist")
            .inc_env_num();
        self.inc_and_reconcile_abrupt_completion_envs();
    }

    /// Tracks that one environment has been pop'ed in the current loop block.
    ///
    /// Note:
    ///  - This will check if the env stack has reached 0 and should be popped.
    pub(crate) fn dec_frame_env_stack(&mut self) {
        //println!("Current place: {}", self.pc);
        //println!("{:#?}\n", self.abrupt_completion);
        //println!("{:#?}\n\n", self.env_stack);
        self.env_stack
            .last_mut()
            .expect("environment stack entry must exist")
            .dec_env_num();
        self.dec_and_reconcile_abrupt_completion_envs();

        if self
            .env_stack
            .last()
            .expect("Must exist as we just decremented it")
            .should_be_popped()
        {
            self.pop_env_stack();
        }
    }

    pub(crate) fn inc_and_reconcile_abrupt_completion_envs(&mut self) {
        if let Some(record) = self.abrupt_completion.as_mut() {
            record.inc_envs();
        }
    }

    pub(crate) fn dec_and_reconcile_abrupt_completion_envs(&mut self) {
        if let Some(record) = self.abrupt_completion.as_mut() {
            if record.envs() > 0 {
                record.dec_envs();
            }
        };
    }

    pub(crate) fn pop_env_stack(&mut self) {
        let dead_entry = self.env_stack.pop().expect("env stack must exist");
        if dead_entry.is_try_env() {
            self.try_catch.pop();
        }
    }
}


/// Tracks the address that should be jumped to when an error is caught.
/// 
/// Additionally the address of a finally block is tracked, to allow for special handling if it exists.
#[derive(Copy, Clone, Debug)]
pub(crate) struct TryAddresses {
    catch: u32,
    finally: Option<u32>,
}

impl TryAddresses {
    pub(crate) fn new(catch_address: u32, finally_address: Option<u32>) -> Self {
        Self {
            catch: catch_address,
            finally: finally_address,
        }
    }
    /// Returns the catch value of the `TryAddresses` struct. 
    pub(crate) const fn catch(&self) -> u32 {
        self.catch
    }

    pub(crate) fn finally(&self) -> Option<u32> {
        self.finally
    }
}

/// Indicates if a function should return or throw at the end of a finally block.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum FinallyReturn {
    None,
    Ok,
    Err,
}

/// Indicates how a generator function that has been called/resumed should return.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum GeneratorResumeKind {
    Normal,
    Throw,
    Return,
}
