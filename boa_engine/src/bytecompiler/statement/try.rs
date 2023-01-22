use boa_ast::{
    declaration::Binding,
    operations::bound_names,
    statement::{Finally, Try},
};

use crate::{
    bytecompiler::ByteCompiler,
    vm::{BindingOpcode, Opcode},
    JsResult,
};

impl ByteCompiler<'_, '_> {
    pub(crate) fn compile_try(
        &mut self,
        t: &Try,
        use_expr: bool,
        configurable_globals: bool,
    ) -> JsResult<()> {
        let (try_start, finally_loc) = self.emit_opcode_with_two_operands(Opcode::TryStart);
        self.patch_jump_with_target(finally_loc, 0);

        // If there is a finally block, initialize the finally control block prior to pushing the try block jump_control
        if t.finally().is_some() {
            self.push_init_finally_control_info();
        }
        self.push_try_control_info(t.finally().is_some(), try_start.index);

        self.context.push_compile_time_environment(false);
        let push_env = self.emit_and_track_decl_env();

        self.create_script_decls(t.block().statement_list(), configurable_globals);
        self.compile_statement_list(t.block().statement_list(), use_expr, configurable_globals)?;

        let (num_bindings, compile_environment) = self.context.pop_compile_time_environment();
        let index_compile_environment = self.push_compile_environment(compile_environment);
        self.patch_jump_with_target(push_env.0, num_bindings as u32);
        self.patch_jump_with_target(push_env.1, index_compile_environment as u32);
        self.emit_and_track_pop_env();
        self.emit_opcode(Opcode::TryEnd);

        let finally = self.jump();
        self.patch_jump(try_start);

        if t.catch().is_some() {
            self.compile_catch_stmt(t, use_expr, configurable_globals)?;
        }

        self.patch_jump(finally);

        if let Some(finally) = t.finally() {
            // Pop and push control loops post FinallyStart, as FinallyStart resets flow control variables.
            // Handle finally header operations
            let finally_start_address = self.next_opcode_location();
            self.emit_opcode(Opcode::FinallyStart);
            self.pop_try_control_info(finally_start_address);
            self.set_jump_control_start_address(finally_start_address);
            self.patch_jump_with_target(finally_loc, finally_start_address);

            // Compile finally statement body
            self.compile_finally_stmt(finally, configurable_globals)?;
        } else {
            let try_end = self.next_opcode_location();
            self.pop_try_control_info(try_end);
        }

        Ok(())
    }

    pub(crate) fn compile_catch_stmt(
        &mut self,
        parent_try: &Try,
        use_expr: bool,
        configurable_globals: bool,
    ) -> JsResult<()> {
        let catch = parent_try
            .catch()
            .expect("Catch must exist for compile_catch_stmt to have been invoked");
        self.set_jump_control_in_catch(true);
        let catch_start = if parent_try.finally().is_some() {
            Some(self.emit_opcode_with_operand(Opcode::CatchStart))
        } else {
            None
        };

        self.context.push_compile_time_environment(false);
        let push_env = self.emit_and_track_decl_env();

        if let Some(binding) = catch.parameter() {
            match binding {
                Binding::Identifier(ident) => {
                    self.context.create_mutable_binding(*ident, false, false);
                    self.emit_binding(BindingOpcode::InitLet, *ident);
                }
                Binding::Pattern(pattern) => {
                    for ident in bound_names(pattern) {
                        self.context.create_mutable_binding(ident, false, false);
                    }
                    self.compile_declaration_pattern(pattern, BindingOpcode::InitLet)?;
                }
            }
        } else {
            self.emit_opcode(Opcode::Pop);
        }

        self.create_script_decls(catch.block().statement_list(), configurable_globals);
        self.compile_statement_list(
            catch.block().statement_list(),
            use_expr,
            configurable_globals,
        )?;

        let (num_bindings, compile_environment) = self.context.pop_compile_time_environment();
        let index_compile_environment = self.push_compile_environment(compile_environment);
        self.patch_jump_with_target(push_env.0, num_bindings as u32);
        self.patch_jump_with_target(push_env.1, index_compile_environment as u32);
        self.emit_and_track_pop_env();
        if let Some(catch_start) = catch_start {
            self.emit_opcode(Opcode::CatchEnd);
            self.patch_jump(catch_start);
        } else {
            self.emit_opcode(Opcode::CatchEnd2);
        }

        self.set_jump_control_in_finally(false);

        Ok(())
    }

    pub(crate) fn compile_finally_stmt(
        &mut self,
        finally: &Finally,
        configurable_globals: bool,
    ) -> JsResult<()> {
        self.context.push_compile_time_environment(false);
        let push_env = self.emit_and_track_decl_env();

        self.create_script_decls(finally.block().statement_list(), configurable_globals);
        self.compile_statement_list(
            finally.block().statement_list(),
            false,
            configurable_globals,
        )?;

        let (num_bindings, compile_environment) = self.context.pop_compile_time_environment();
        let index_compile_environment = self.push_compile_environment(compile_environment);
        self.patch_jump_with_target(push_env.0, num_bindings as u32);
        self.patch_jump_with_target(push_env.1, index_compile_environment as u32);

        self.emit_and_track_pop_env();
        self.pop_finally_control_info();
        self.emit_opcode(Opcode::FinallyEnd);

        Ok(())
    }
}
