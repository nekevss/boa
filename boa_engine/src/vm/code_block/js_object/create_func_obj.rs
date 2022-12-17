
use crate::{
    builtins::{
        function::{
            ConstructorKind, Function,
        },
        promise::PromiseCapability,
    },
    js_string,
    object::{
        JsObject, ObjectData,
    },
    property::PropertyDescriptor,
    vm::CodeBlock,
    Context, JsString,
};
use boa_gc::Gc;
use boa_profiler::Profiler;

impl JsObject {
    /// Creates a new function object.
    pub(crate) fn create_function_object(
        code: Gc<CodeBlock>,
        r#async: bool,
        arrow: bool,
        prototype: Option<JsObject>,
        context: &mut Context,
    ) -> Self {
        let _timer = Profiler::global().start_event("JsVmFunction::new", "vm");

        let function_prototype = if let Some(prototype) = prototype {
            prototype
        } else if r#async {
            context
                .intrinsics()
                .constructors()
                .async_function()
                .prototype()
        } else {
            context.intrinsics().constructors().function().prototype()
        };

        let prototype = context.construct_object();

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

        let function = if r#async {
            let promise_capability = PromiseCapability::new(
                &context
                    .intrinsics()
                    .constructors()
                    .promise()
                    .constructor()
                    .into(),
                context,
            )
            .expect("cannot  fail per spec");

            Function::Async {
                code,
                environments: context.realm.environments.clone(),
                home_object: None,
                promise_capability,
            }
        } else {
            Function::Ordinary {
                code,
                environments: context.realm.environments.clone(),
                constructor_kind: ConstructorKind::Base,
                home_object: None,
                fields: Vec::new(),
                private_methods: Vec::new(),
            }
        };

        let constructor =
            JsObject::from_proto_and_data(function_prototype, ObjectData::function(function));

        let constructor_property = PropertyDescriptor::builder()
            .value(constructor.clone())
            .writable(true)
            .enumerable(false)
            .configurable(true)
            .build();

        prototype
            .define_property_or_throw(js_string!("constructor"), constructor_property, context)
            .expect("failed to define the constructor property of the function");

        let prototype_property = PropertyDescriptor::builder()
            .value(prototype)
            .writable(true)
            .enumerable(false)
            .configurable(false)
            .build();

        constructor
            .define_property_or_throw(js_string!("length"), length_property, context)
            .expect("failed to define the length property of the function");
        constructor
            .define_property_or_throw(js_string!("name"), name_property, context)
            .expect("failed to define the name property of the function");
        if !r#async && !arrow {
            constructor
                .define_property_or_throw(js_string!("prototype"), prototype_property, context)
                .expect("failed to define the prototype property of the function");
        }

        constructor
    }
}