use crate::{
    builtins::function::ClassFieldDefinition,
    object::{
        JsObject, PrivateElement,
    },
    property::PropertyDescriptor,
    Context, JsResult, 
};

impl JsObject {
    /// `InitializeInstanceElements ( O, constructor )`
    ///
    /// Add private methods and fields from a class constructor to an object.
    ///
    /// More information:
    ///  - [ECMAScript specification][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-initializeinstanceelements
    pub(crate) fn initialize_instance_elements(
        &self,
        constructor: &JsObject,
        context: &mut Context,
    ) -> JsResult<()> {
        let constructor_borrow = constructor.borrow();
        let constructor_function = constructor_borrow
            .as_function()
            .expect("class constructor must be function object");

        for (name, private_method) in constructor_function.get_private_methods() {
            match private_method {
                PrivateElement::Method(_) => {
                    self
                        .borrow_mut()
                        .set_private_element(*name, private_method.clone());
                }
                PrivateElement::Accessor { getter, setter } => {
                    if let Some(getter) = getter {
                        self
                            .borrow_mut()
                            .set_private_element_getter(*name, getter.clone());
                    }
                    if let Some(setter) = setter {
                        self
                            .borrow_mut()
                            .set_private_element_setter(*name, setter.clone());
                    }
                }
                PrivateElement::Field(_) => unreachable!(),
            }
        }

        for field in constructor_function.get_fields() {
            match field {
                ClassFieldDefinition::Public(name, function) => {
                    let value = function.call(&self.clone().into(), &[], context)?;
                    self.__define_own_property__(
                        name.clone(),
                        PropertyDescriptor::builder()
                            .value(value)
                            .writable(true)
                            .enumerable(true)
                            .configurable(true)
                            .build(),
                        context,
                    )?;
                }
                ClassFieldDefinition::Private(name, function) => {
                    let value = function.call(&self.clone().into(), &[], context)?;
                    self
                        .borrow_mut()
                        .set_private_element(*name, PrivateElement::Field(value));
                }
            }
        }

        Ok(())
    }
}
