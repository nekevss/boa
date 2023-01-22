//! `CallFrame`
//!
//! This module will provides everything needed to implement the `CallFrame`

use crate::{object::JsObject, vm::CodeBlock};
use boa_gc::{Finalize, Gc, Trace};

/// A `CallFrame` holds the state of a function call.
#[derive(Clone, Debug, Finalize, Trace)]
pub struct CallFrame {
    pub(crate) code_block: Gc<CodeBlock>,
    pub(crate) pc: usize,
    #[unsafe_ignore_trace]
    pub(crate) catch: Vec<CatchAddresses>,
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
            catch: Vec::new(),
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
        self.inc_abrupt_completion_envs();
    }

    /// Tracks that one environment has been pop'ed in the current loop block.
    ///
    /// Note:
    ///  - This will check if the env stack has reached 0 and should be popped.
    pub(crate) fn dec_frame_env_stack(&mut self) {
        self.env_stack
            .last_mut()
            .expect("environment stack entry must exist")
            .dec_env_num();
        self.dec_abrupt_completion_envs();

        if self
            .env_stack
            .last()
            .expect("Must exist as we just decremented it")
            .should_be_popped()
        {
            self.pop_env_stack();
        }
    }

    pub(crate) fn inc_abrupt_completion_envs(&mut self) {
        if let Some(record) = self.abrupt_completion.as_mut() {
            record.inc_envs_to_reconcile();
        }
    }

    pub(crate) fn dec_abrupt_completion_envs(&mut self) {
        if let Some(record) = self.abrupt_completion.as_mut() {
            record.dec_envs_to_reconcile();
        }
    }

    pub(crate) fn pop_env_stack(&mut self) {
        let dead_entry = self.env_stack.pop().expect("env stack must exist");
        if dead_entry.is_try_env() {
            self.catch.pop();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum EnvEntryKind {
    Global,
    Loop,
    Try,
    Catch,
    Finally,
}
/// Tracks the number of environments in the current try-catch-finally block.
///
/// Because of the interactions between loops and try-catch-finally blocks,
/// the number of loop blocks in the try-catch-finally block also needs to be tracked.
#[derive(Copy, Clone, Debug)]
pub(crate) struct EnvStackEntry {
    kind: EnvEntryKind,
    env_num: usize,
}

impl Default for EnvStackEntry {
    fn default() -> Self {
        Self {
            kind: EnvEntryKind::Global,
            env_num: 0,
        }
    }
}

/// ---- `EnvStackEntry` creation methods ----
impl EnvStackEntry {
    pub(crate) const fn with_try_flag(mut self) -> Self {
        self.kind = EnvEntryKind::Try;
        self
    }

    pub(crate) const fn with_loop_flag(mut self) -> Self {
        self.kind = EnvEntryKind::Loop;
        self
    }

    pub(crate) const fn with_catch_flag(mut self) -> Self {
        self.kind = EnvEntryKind::Catch;
        self
    }

    pub(crate) const fn with_finally_flag(mut self) -> Self {
        self.kind = EnvEntryKind::Finally;
        self
    }

    pub(crate) const fn with_initial_env_num(mut self, value: usize) -> Self {
        self.env_num = value;
        self
    }
}

/// ---- `EnvStackEntry` interaction methods ----
impl EnvStackEntry {
    /// Returns true if an `EnvStackEntry` is a loop
    pub(crate) fn is_loop_env(&self) -> bool {
        self.kind == EnvEntryKind::Loop
    }

    /// Returns true if an `EnvStackEntry` is a try block
    pub(crate) fn is_try_env(&self) -> bool {
        self.kind == EnvEntryKind::Try
    }

    /// Returns the current environment number for this entry.
    pub(crate) const fn env_num(&self) -> usize {
        self.env_num
    }

    /// Checks if a env block should be popped
    pub(crate) fn should_be_popped(&self) -> bool {
        self.kind != EnvEntryKind::Global && self.env_num == 0
    }

    pub(crate) fn inc_env_num(&mut self) {
        self.env_num += 1;
    }

    pub(crate) fn dec_env_num(&mut self) {
        self.env_num -= 1;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum AbruptKind {
    None,
    Continue,
    Break,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct AbruptCompletionRecord {
    kind: AbruptKind,
    target: u32,
    envs: u32,
}

impl Default for AbruptCompletionRecord {
    fn default() -> Self {
        Self {
            kind: AbruptKind::None,
            target: u32::MAX,
            envs: 0,
        }
    }
}

/// ---- `AbruptCompletionRecord` initialization methods ----
impl AbruptCompletionRecord {
    pub(crate) const fn with_break_flag(mut self) -> Self {
        self.kind = AbruptKind::Break;
        self
    }

    pub(crate) const fn with_continue_flag(mut self) -> Self {
        self.kind = AbruptKind::Continue;
        self
    }

    pub(crate) const fn with_initial_target(mut self, target: u32) -> Self {
        self.target = target;
        self
    }

    pub(crate) const fn with_initial_envs(mut self, envs: u32) -> Self {
        self.envs = envs;
        self
    }
}

impl AbruptCompletionRecord {
    pub(crate) fn is_break(&self) -> bool {
        self.kind == AbruptKind::Break
    }

    pub(crate) const fn target(&self) -> u32 {
        self.target
    }

    pub(crate) const fn envs(&self) -> u32 {
        self.envs
    }
}

/// ---- `AbruptCompletionRecord` interaction methods ----
impl AbruptCompletionRecord {
    pub(crate) fn set_break_flag(&mut self) {
        self.kind = AbruptKind::Break;
    }

    pub(crate) fn set_continue_flag(&mut self) {
        self.kind = AbruptKind::Continue;
    }

    pub(crate) fn set_target(&mut self, target: u32) {
        self.target = target;
    }

    pub(crate) fn set_envs(&mut self, envs: u32) {
        self.envs = envs;
    }

    pub(crate) fn inc_envs_to_reconcile(&mut self) {
        self.envs += 1;
    }

    pub(crate) fn dec_envs_to_reconcile(&mut self) {
        self.envs -= 1;
    }
}

/// Tracks the address that should be jumped to when an error is caught.
/// Additionally the address of a finally block is tracked, to allow for special handling if it exists.
#[derive(Copy, Clone, Debug)]
pub(crate) struct CatchAddresses {
    pub(crate) next: u32,
    pub(crate) finally: Option<u32>,
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
