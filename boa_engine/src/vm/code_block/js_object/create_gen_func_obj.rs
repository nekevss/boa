use crate::{
    builtins::function::Function,
    js_string,
    object::{
        JsObject, ObjectData,
    },
    property::PropertyDescriptor,
    vm::CodeBlock,
    Context, JsString,
};
use boa_gc::Gc;

impl JsObject {
    /// Creates a new generator function object.
    pub(crate) fn create_generator_function_object(
        code: Gc<CodeBlock>,
        r#async: bool,
        context: &mut Context,
    ) -> Self {
        let function_prototype = if r#async {
            context
                .intrinsics()
                .constructors()
                .async_generator_function()
                .prototype()
        } else {
            context
                .intrinsics()
                .constructors()
                .generator_function()
                .prototype()
        };

        let name_property = PropertyDescriptor::builder()
            .value(
                context
                    .interner()
                    .resolve_expect(code.name)
                    .into_common::<JsString>(false),
            )
            .writable(false)
            .enumerable(false)
            .configurable(true)
            .build();

        let length_property = PropertyDescriptor::builder()
            .value(code.length)
            .writable(false)
            .enumerable(false)
            .configurable(true)
            .build();

        let prototype = JsObject::from_proto_and_data(
            if r#async {
                context
                    .intrinsics()
                    .constructors()
                    .async_generator()
                    .prototype()
            } else {
                context.intrinsics().constructors().generator().prototype()
            },
            ObjectData::ordinary(),
        );

        let constructor = if r#async {
            let function = Function::AsyncGenerator {
                code,
                environments: context.realm.environments.clone(),
                home_object: None,
            };
            JsObject::from_proto_and_data(
                function_prototype,
                ObjectData::async_generator_function(function),
            )
        } else {
            let function = Function::Generator {
                code,
                environments: context.realm.environments.clone(),
                home_object: None,
            };
            JsObject::from_proto_and_data(function_prototype, ObjectData::generator_function(function))
        };

        let prototype_property = PropertyDescriptor::builder()
            .value(prototype)
            .writable(true)
            .enumerable(false)
            .configurable(false)
            .build();

        constructor
            .define_property_or_throw(js_string!("prototype"), prototype_property, context)
            .expect("failed to define the prototype property of the generator function");
        constructor
            .define_property_or_throw(js_string!("name"), name_property, context)
            .expect("failed to define the name property of the generator function");
        constructor
            .define_property_or_throw(js_string!("length"), length_property, context)
            .expect("failed to define the length property of the generator function");

        constructor
    }
}
