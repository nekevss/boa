//! `JumpControlInfo` tracks relevant jump information used during compilation.
//!
//! Primarily, jump control tracks information related to the compilation of [iteration
//! statements][iteration spec], [switch statements][switch spec], [try statements][try spec],
//! and [labelled statements][labelled spec].
//!
//! [iteration spec]: https://tc39.es/ecma262/#sec-iteration-statements
//! [switch spec]: https://tc39.es/ecma262/#sec-switch-statement
//! [try spec]: https://tc39.es/ecma262/#sec-try-statement
//! [labelled spec]: https://tc39.es/ecma262/#sec-labelled-statements

use crate::{
    bytecompiler::{ByteCompiler, Label},
    vm::Opcode, JsResult,
};
use bitflags::bitflags;
use boa_interner::Sym;

/// Boa's `ByteCompiler` jump information tracking struct.
#[derive(Debug, Clone)]
pub(crate) struct JumpControlInfo {
    label: Option<Sym>,
    start_address: u32,
    decl_envs: u32,
    flags: JumpControlInfoFlags,
    breaks: Vec<Label>,
    try_continues: Vec<Label>,
}

bitflags! {
    /// A bitflag that contains the type flags and relevant booleans for `JumpControlInfo`.
    pub(crate) struct JumpControlInfoFlags: u8 {
        const LOOP = 0b0000_0001;
        const SWITCH = 0b0000_0010;
        const TRY_BLOCK = 0b0000_0100;
        const LABELLED = 0b0000_1000;
        const IN_CATCH = 0b0001_0000;
        const IN_FINALLY = 0b0010_0000;
        const HAS_FINALLY = 0b0100_0000;
        const FOR_OF_IN_LOOP = 0b1000_0000;
    }
}

impl Default for JumpControlInfoFlags {
    fn default() -> Self {
        Self::empty()
    }
}

impl Default for JumpControlInfo {
    fn default() -> Self {
        Self {
            label: None,
            start_address: u32::MAX,
            decl_envs: 0,
            flags: JumpControlInfoFlags::default(),
            breaks: Vec::new(),
            try_continues: Vec::new(),
        }
    }
}

/// ---- `JumpControlInfo` Creation Methods ----
impl JumpControlInfo {
    pub(crate) const fn with_label(mut self, label: Option<Sym>) -> Self {
        self.label = label;
        self
    }

    pub(crate) const fn with_start_address(mut self, address: u32) -> Self {
        self.start_address = address;
        self
    }

    pub(crate) fn with_loop_flag(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::LOOP, value);
        self
    }

    pub(crate) fn with_switch_flag(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::SWITCH, value);
        self
    }

    pub(crate) fn with_try_block_flag(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::TRY_BLOCK, value);
        self
    }

    pub(crate) fn with_labelled_block_flag(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::LABELLED, value);
        self
    }

    pub(crate) fn with_has_finally(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::HAS_FINALLY, value);
        self
    }

    pub(crate) fn with_for_of_in_loop(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::FOR_OF_IN_LOOP, value);
        self
    }
}

/// ---- `JumpControlInfo` const fn methods ----
impl JumpControlInfo {
    pub(crate) const fn label(&self) -> Option<Sym> {
        self.label
    }

    pub(crate) const fn start_address(&self) -> u32 {
        self.start_address
    }

    pub(crate) const fn is_loop(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::LOOP)
    }

    pub(crate) const fn is_switch(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::SWITCH)
    }

    pub(crate) const fn is_try_block(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::TRY_BLOCK)
    }

    pub(crate) const fn is_labelled(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::LABELLED)
    }

    pub(crate) const fn in_catch(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::IN_CATCH)
    }

    pub(crate) const fn in_finally(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::IN_FINALLY)
    }

    pub(crate) const fn has_finally(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::HAS_FINALLY)
    }

    pub(crate) const fn for_of_in_loop(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::FOR_OF_IN_LOOP)
    }

    pub(crate) const fn decl_envs(&self) -> u32 {
        self.decl_envs
    }
}

/// ---- `JumpControlInfo` interaction methods ----
impl JumpControlInfo {
    /// Sets the `label` field of `JumpControlInfo`.
    pub(crate) fn set_label(&mut self, label: Option<Sym>) {
        assert!(self.label.is_none());
        self.label = label;
    }

    /// Sets the `start_address` field of `JumpControlInfo`.
    pub(crate) fn set_start_address(&mut self, start_address: u32) {
        self.start_address = start_address;
    }

    /// Sets the `in_catch` field of `JumpControlInfo`.
    pub(crate) fn set_in_catch(&mut self, value: bool) {
        self.flags.set(JumpControlInfoFlags::IN_CATCH, value);
    }

    /// Set the `in_finally` field of `JumpControlInfo`.
    pub(crate) fn set_in_finally(&mut self, value: bool) {
        self.flags.set(JumpControlInfoFlags::IN_FINALLY, value);
    }

    /// Sets the `finally_start` field of `JumpControlInfo`.
    pub(crate) fn set_finally_start(&mut self, label: Label) {
        self.finally_start = Some(label);
    }

    /// Increments the `decl_env` field of `JumpControlInfo`.
    pub(crate) fn inc_decl_envs(&mut self) {
        self.decl_envs += 1;
    }

    /// Decrements the `decl_env` field of `JumpControlInfo`.
    pub(crate) fn dec_decl_envs(&mut self) {
        self.decl_envs -= 1;
    }

    /// Pushes a `Label` onto the `break` vector of `JumpControlInfo`.
    pub(crate) fn push_break_label(&mut self, break_label: Label) {
        self.breaks.push(break_label);
    }

    /// Pushes a `Label` onto the `try_continues` vector of `JumpControlInfo`.
    pub(crate) fn push_try_continue_label(&mut self, try_continue_label: Label) {
        self.try_continues.push(try_continue_label);
    }
}

// `JumpControlInfo` related methods that are implemented on `ByteCompiler`.
impl ByteCompiler<'_, '_> {
    /// Pushes a generic `JumpControlInfo` onto `ByteCompiler`
    ///
    /// Default `JumpControlInfoKind` is `JumpControlInfoKind::Loop`
    pub(crate) fn push_empty_loop_jump_control(&mut self) {
        self.jump_info
            .push(JumpControlInfo::default().with_loop_flag(true));
    }

    pub(crate) fn current_jump_control_mut(&mut self) -> Option<&mut JumpControlInfo> {
        self.jump_info.last_mut()
    }

    pub(crate) fn set_jump_control_in_finally(&mut self, value: bool) {
        if !self.jump_info.is_empty() {
            let info = self
                .jump_info
                .last_mut()
                .expect("must have try control label");
            assert!(info.is_try_block());
            info.set_in_finally(value);
        }
    }

    pub(crate) fn set_jump_control_in_catch(&mut self, value: bool) {
        if !self.jump_info.is_empty() {
            let info = self
                .jump_info
                .last_mut()
                .expect("must have try control label");
            assert!(info.is_try_block());
            info.set_in_catch(value);
        }
    }

    /// Emits the `PushDeclarativeEnvironment` and updates the current jump info to track environments.
    pub(crate) fn emit_and_track_decl_env(&mut self) -> (Label, Label) {
        let pushed_env = self.emit_opcode_with_two_operands(Opcode::PushDeclarativeEnvironment);
        if !self.jump_info.is_empty() {
            let current_jump_info = self
                .jump_info
                .last_mut()
                .expect("Jump info must exist as the vector is not empty");
            current_jump_info.inc_decl_envs();
        }
        pushed_env
    }

    /// Emits the `PopEnvironment` Opcode and updates the current jump that the env is removed.
    pub(crate) fn emit_and_track_pop_env(&mut self) {
        self.emit_opcode(Opcode::PopEnvironment);
        if !self.jump_info.is_empty() {
            let current_info = self.jump_info.last_mut().expect("JumpInfo must exist");
            current_info.dec_decl_envs();
        }
    }

    // ---- Labelled Statement JumpControlInfo methods ---- //

    /// Pushes a `LabelledStatement`'s `JumpControlInfo` onto the `jump_info` stack.
    pub(crate) fn push_labelled_control_info(&mut self, label: Sym, start_address: u32) {
        let new_info = JumpControlInfo::default()
            .with_labelled_block_flag(true)
            .with_label(Some(label))
            .with_start_address(start_address);
        self.jump_info.push(new_info);
    }

    /// Pops and handles the info for a label's `JumpControlInfo`
    /// 
    /// # Panic
    ///  - Will panic if `jump_info` stack is empty.
    ///  - Will panic if popped `JumpControlInfo` is not for a `LabelledStatement`.
    pub(crate) fn pop_labelled_control_info(&mut self) {
        assert!(!self.jump_info.is_empty());
        let info = self.jump_info.pop().expect("no jump information found");

        assert!(info.is_labelled());

        for label in info.breaks {
            self.patch_jump(label);
        }

        for label in info.try_continues {
            self.patch_jump_with_target(label, info.start_address);
        }
    }
    // ---- `IterationStatement`'s `JumpControlInfo` methods ---- //

    /// Pushes an `WhileStatement`, `ForStatement` or `DoWhileStatement`'s `JumpControlInfo` on to the `jump_info` stack.
    pub(crate) fn push_loop_control_info(&mut self, label: Option<Sym>, start_address: u32) {
        let new_info = JumpControlInfo::default()
            .with_loop_flag(true)
            .with_label(label)
            .with_start_address(start_address);
        self.jump_info.push(new_info);
    }

    /// Pushes a `ForInOfStatement`'s `JumpControlInfo` on to the `jump_info` stack.
    pub(crate) fn push_loop_control_info_for_of_in_loop(
        &mut self,
        label: Option<Sym>,
        start_address: u32,
    ) {
        let new_info = JumpControlInfo::default()
            .with_loop_flag(true)
            .with_label(label)
            .with_start_address(start_address)
            .with_for_of_in_loop(true);
        self.jump_info.push(new_info);
    }

    /// Pops and handles the info for a loop control block's `JumpControlInfo`
    /// 
    /// # Panic
    ///  - Will panic if `jump_info` stack is empty.
    ///  - Will panic if popped `JumpControlInfo` is not for a loop block.
    pub(crate) fn pop_loop_control_info(&mut self) {
        assert!(!self.jump_info.is_empty());
        let info = self.jump_info.pop().expect("no jump information found");

        assert!(info.is_loop());


        for label in info.breaks {
            self.patch_jump(label);
        }

        for label in info.try_continues {
            self.patch_jump_with_target(label, info.start_address);
        }
    }

    // ---- `SwitchStatement` `JumpControlInfo` methods ---- //

    /// Pushes a `SwitchStatement`'s `JumpControlInfo` on to the `jump_info` stack.
    pub(crate) fn push_switch_control_info(&mut self, label: Option<Sym>, start_address: u32) {
        let new_info = JumpControlInfo::default()
            .with_switch_flag(true)
            .with_label(label)
            .with_start_address(start_address);
        self.jump_info.push(new_info);
    }

    /// Pops and handles the info for a switch block's `JumpControlInfo`
    /// 
    /// # Panic
    ///  - Will panic if `jump_info` stack is empty.
    ///  - Will panic if popped `JumpControlInfo` is not for a switch block.
    pub(crate) fn pop_switch_control_info(&mut self) {
        assert!(!self.jump_info.is_empty());
        let info = self.jump_info.pop().expect("no jump information found");

        assert!(info.is_switch());

        for label in info.breaks {
            self.patch_jump(label);
        }
    }

    // ---- `TryStatement`'s `JumpControlInfo` methods ---- //

    /// Pushes a `TryStatement`'s `JumpControlInfo` onto the `jump_info` stack.
    pub(crate) fn push_try_control_info(&mut self, has_finally: bool, start_address: u32) {
        let new_info = JumpControlInfo::default()
            .with_try_block_flag(true)
            .with_start_address(start_address)
            .with_has_finally(has_finally);

        self.jump_info.push(new_info);
    }

    /// Pops and handles the info for a try block's `JumpControlInfo`
    /// 
    /// # Panic
    ///  - Will panic if `jump_info` is empty.
    ///  - Will panic if popped `JumpControlInfo` is not for a try block.
    pub(crate) fn pop_try_control_info(&mut self, finally_start_address: Option<u32>) -> JsResult<()> {
        assert!(!self.jump_info.is_empty());
        let mut info = self.jump_info.pop().expect("no jump information found");

        assert!(info.is_try_block());

        let mut breaks = Vec::with_capacity(info.breaks.len());

        if let Some(finally_start_address) = finally_start_address {
            for label in &info.try_continues {
                if label.index < finally_start_address {
                    self.patch_jump_with_target(*label, finally_start_address);
                } else {
                    self.patch_jump_with_target(*label, info.start_address);
                }
            }

            for label in info.breaks {
                if label.index < finally_start_address {
                    self.patch_jump_with_target(label, finally_start_address);
                    let finally_jump = self.emit_opcode_with_operand(Opcode::FinallySetJump);
                    breaks.push(finally_jump);
                } else {
                    breaks.push(label)
                }
            }
            if let Some(jump_info) = self.jump_info.last_mut() {
                jump_info.breaks.append(&mut breaks);
            }
        } else if let Some(jump_info) = self.jump_info.last_mut() {
            jump_info.breaks.append(&mut info.breaks);
            jump_info.try_continues.append(&mut info.try_continues);
        }

        Ok(())
    }

    /// Pushes a `TryStatement`'s Finally block `JumpControlInfo` onto the `jump_info` stack.
    pub(crate) fn push_finally_control_info(&mut self, start_address: u32) {
        let new_info = JumpControlInfo::default()
            .with_try_block_flag(true)
            .with_start_address(start_address)
            .set_in_finally(true);

        self.jump_info.push(new_info);
    }

    pub(crate) fn pop_finally_control_info(&mut self) {
        assert!(!self.jump_info.is_empty());
        let info = self.jump_info.pop().expect("no jump information found");

        assert!(info.in_finally());


        for label in info.breaks {
            self.patch_jump(label);
        }

        for label in info.try_continues {
            self.patch_jump_with_target(label, info.start_address);
        }
    }
}
