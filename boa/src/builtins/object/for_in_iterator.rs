use crate::{
    builtins::{function::make_builtin_fn, iterable::create_iter_result_object},
    gc::{Finalize, Trace},
    object::{JsObject, ObjectData},
    property::PropertyDescriptor,
    property::PropertyKey,
    symbol::WellKnownSymbols,
    BoaProfiler, Context, JsResult, JsString, JsValue,
};
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

/// The ForInIterator object represents an iteration over some specific object.
/// It implements the iterator protocol.
///
/// More information:
///  - [ECMAScript reference][spec]
///
/// [spec]: https://tc39.es/ecma262/#sec-for-in-iterator-objects
#[derive(Debug, Clone, Finalize, Trace)]
pub struct ForInIterator {
    object: JsValue,
    visited_keys: FxHashSet<JsString>,
    remaining_keys: VecDeque<JsString>,
    object_was_visited: bool,
}

impl ForInIterator {
    pub(crate) const NAME: &'static str = "ForInIterator";

    fn new(object: JsValue) -> Self {
        ForInIterator {
            object,
            visited_keys: FxHashSet::default(),
            remaining_keys: VecDeque::default(),
            object_was_visited: false,
        }
    }

    /// CreateForInIterator( object )
    ///
    /// Creates a new iterator over the given object.
    ///
    /// More information:
    ///  - [ECMA reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-createforiniterator
    pub(crate) fn create_for_in_iterator(object: JsValue, context: &Context) -> JsValue {
        let for_in_iterator = JsValue::new_object(context);
        for_in_iterator.set_data(ObjectData::for_in_iterator(Self::new(object)));
        for_in_iterator
            .as_object()
            .expect("for in iterator object")
            .set_prototype_instance(context.iterator_prototypes().for_in_iterator().into());
        for_in_iterator
    }

    /// %ForInIteratorPrototype%.next( )
    ///
    /// Gets the next result in the object.
    ///
    /// More information:
    ///  - [ECMA reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-%foriniteratorprototype%.next
    pub(crate) fn next(this: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let mut for_in_iterator = this.as_object().map(|obj| obj.borrow_mut());
        let for_in_iterator = for_in_iterator
            .as_mut()
            .and_then(|obj| obj.as_for_in_iterator_mut())
            .ok_or_else(|| context.construct_type_error("`this` is not a ForInIterator"))?;
        let mut object = for_in_iterator.object.to_object(context)?;
        loop {
            if !for_in_iterator.object_was_visited {
                let keys = object.__own_property_keys__(context)?;
                for k in keys {
                    match k {
                        PropertyKey::String(ref k) => {
                            for_in_iterator.remaining_keys.push_back(k.clone());
                        }
                        PropertyKey::Index(i) => {
                            for_in_iterator
                                .remaining_keys
                                .push_back(i.to_string().into());
                        }
                        _ => {}
                    }
                }
                for_in_iterator.object_was_visited = true;
            }
            while let Some(r) = for_in_iterator.remaining_keys.pop_front() {
                if !for_in_iterator.visited_keys.contains(&r) {
                    if let Some(desc) =
                        object.__get_own_property__(&PropertyKey::from(r.clone()), context)?
                    {
                        for_in_iterator.visited_keys.insert(r.clone());
                        if desc.expect_enumerable() {
                            return Ok(create_iter_result_object(
                                JsValue::new(r.to_string()),
                                false,
                                context,
                            ));
                        }
                    }
                }
            }
            match object.prototype_instance().to_object(context) {
                Ok(o) => {
                    object = o;
                }
                _ => {
                    return Ok(create_iter_result_object(
                        JsValue::undefined(),
                        true,
                        context,
                    ))
                }
            }
            for_in_iterator.object = JsValue::new(object.clone());
            for_in_iterator.object_was_visited = false;
        }
    }

    /// Create the %ArrayIteratorPrototype% object
    ///
    /// More information:
    ///  - [ECMA reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-%foriniteratorprototype%-object
    pub(crate) fn create_prototype(iterator_prototype: JsValue, context: &mut Context) -> JsObject {
        let _timer = BoaProfiler::global().start_event(Self::NAME, "init");

        // Create prototype
        let for_in_iterator = context.construct_object();
        make_builtin_fn(Self::next, "next", &for_in_iterator, 0, context);
        for_in_iterator.set_prototype_instance(iterator_prototype);

        let to_string_tag = WellKnownSymbols::to_string_tag();
        let to_string_tag_property = PropertyDescriptor::builder()
            .value("For In Iterator")
            .writable(false)
            .enumerable(false)
            .configurable(true);
        for_in_iterator.insert(to_string_tag, to_string_tag_property);
        for_in_iterator
    }
}
