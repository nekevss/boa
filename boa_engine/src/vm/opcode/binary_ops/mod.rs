use crate::{
    vm::{opcode::Operation, ShouldExit},
    Context, JsResult, JsValue,
};

pub(crate) mod macro_defined;
pub(crate) mod logical;

pub(crate) use macro_defined::*;
pub(crate) use logical::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NotEq;

impl Operation for NotEq {
    const NAME: &'static str = "NotEq";
    const INSTRUCTION: &'static str = "INST - NotEq";

    fn execute(context: &mut Context) -> JsResult<ShouldExit> {
        let rhs = context.vm.pop();
        let lhs = context.vm.pop();
        let value = !lhs.equals(&rhs, context)?;
        context.vm.push(value);
        Ok(ShouldExit::False)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct StrictEq;

impl Operation for StrictEq {
    const NAME: &'static str = "StrictEq";
    const INSTRUCTION: &'static str = "INST - StrictEq";

    fn execute(context: &mut Context) -> JsResult<ShouldExit> {
        let rhs = context.vm.pop();
        let lhs = context.vm.pop();
        context.vm.push(lhs.strict_equals(&rhs));
        Ok(ShouldExit::False)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct StrictNotEq;

impl Operation for StrictNotEq {
    const NAME: &'static str = "StrictNotEq";
    const INSTRUCTION: &'static str = "INST - StrictNotEq";

    fn execute(context: &mut Context) -> JsResult<ShouldExit> {
        let rhs = context.vm.pop();
        let lhs = context.vm.pop();
        context.vm.push(!lhs.strict_equals(&rhs));
        Ok(ShouldExit::False)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct In;

impl Operation for In {
    const NAME: &'static str = "In";
    const INSTRUCTION: &'static str = "INST - In";

    fn execute(context: &mut Context) -> JsResult<ShouldExit> {
        let rhs = context.vm.pop();
        let lhs = context.vm.pop();

        if !rhs.is_object() {
            return context.throw_type_error(format!(
                "right-hand side of 'in' should be an object, got {}",
                rhs.type_of().to_std_string_escaped()
            ));
        }
        let key = lhs.to_property_key(context)?;
        let value = context.has_property(&rhs, &key)?;
        context.vm.push(value);
        Ok(ShouldExit::False)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct InstanceOf;

impl Operation for InstanceOf {
    const NAME: &'static str = "InstanceOf";
    const INSTRUCTION: &'static str = "INST - InstanceOf";

    fn execute(context: &mut Context) -> JsResult<ShouldExit> {
        let target = context.vm.pop();
        let v = context.vm.pop();
        let value = v.instance_of(&target, context)?;

        context.vm.push(value);
        Ok(ShouldExit::False)
    }
}

