//! <https://tc39.es/ecma262/#sec-typedarray-objects>

pub use self::integer_indexed_object::IntegerIndexed;
use crate::{
    builtins::{
        array_buffer::{ArrayBuffer, SharedMemoryOrder},
        iterable::iterable_to_list,
        typed_array::integer_indexed_object::ContentType,
        Array, BuiltIn, JsArgs,
    },
    context::{StandardConstructor, StandardObjects},
    gc::{empty_trace, Finalize, Trace},
    object::{
        internal_methods::get_prototype_from_constructor, ConstructorBuilder, FunctionBuilder,
        JsObject, ObjectData,
    },
    property::Attribute,
    symbol::WellKnownSymbols,
    value::JsValue,
    BoaProfiler, Context, JsResult,
};

pub mod integer_indexed_object;

macro_rules! typed_array {
    ($ty:ident, $name:literal, $global_object_name:ident) => {
        #[doc = concat!("JavaScript `", $name, "` built-in implementation.")]
        #[derive(Debug, Clone, Copy)]
        pub struct $ty;

        impl BuiltIn for $ty {
            const NAME: &'static str = $name;

            fn attribute() -> Attribute {
                Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::CONFIGURABLE
            }

            fn init(context: &mut Context) -> (&'static str, JsValue, Attribute) {
                let _timer = BoaProfiler::global().start_event(Self::NAME, "init");

                let get_species = FunctionBuilder::native(context, TypedArray::get_species)
                    .name("get [Symbol.species]")
                    .constructable(false)
                    .build();

                let typed_array = ConstructorBuilder::with_standard_object(
                    context,
                    Self::constructor,
                    context.standard_objects().$global_object_name().clone(),
                )
                .name(Self::NAME)
                .length(Self::LENGTH)
                .static_accessor(
                    WellKnownSymbols::species(),
                    Some(get_species),
                    None,
                    Attribute::CONFIGURABLE,
                )
                .property(
                    "BYTES_PER_ELEMENT",
                    TypedArrayName::$ty.element_size(),
                    Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT,
                )
                .static_property(
                    "BYTES_PER_ELEMENT",
                    TypedArrayName::$ty.element_size(),
                    Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT,
                )
                .build();

                (Self::NAME, typed_array.into(), Self::attribute())
            }
        }

        impl $ty {
            const LENGTH: usize = 3;

            /// `23.2.5.1 TypedArray ( ...args )`
            ///
            /// More information:
            ///  - [ECMAScript reference][spec]
            ///
            /// [spec]: https://tc39.es/ecma262/#sec-typedarray
            fn constructor(
                new_target: &JsValue,
                args: &[JsValue],
                context: &mut Context,
            ) -> JsResult<JsValue> {
                // 1. If NewTarget is undefined, throw a TypeError exception.
                if new_target.is_undefined() {
                    return context.throw_type_error(concat!(
                        "new target was undefined when constructing an ",
                        $name
                    ));
                }

                // 2. Let constructorName be the String value of the Constructor Name value specified in Table 72 for this TypedArray constructor.
                let constructor_name = TypedArrayName::$ty;

                // 3. Let proto be "%TypedArray.prototype%".
                let proto = StandardObjects::$global_object_name;

                // 4. Let numberOfArgs be the number of elements in args.
                let number_of_args = args.len();

                // 5. If numberOfArgs = 0, then
                if number_of_args == 0 {
                    // a. Return ? AllocateTypedArray(constructorName, NewTarget, proto, 0).
                    return Ok(TypedArray::allocate(
                        constructor_name,
                        new_target,
                        proto,
                        Some(0),
                        context,
                    )?
                    .into());
                }
                // 6. Else,

                // a. Let firstArgument be args[0].
                let first_argument = &args[0];

                // b. If Type(firstArgument) is Object, then
                if let Some(first_argument) = first_argument.as_object() {
                    // i. Let O be ? AllocateTypedArray(constructorName, NewTarget, proto).
                    let o =
                        TypedArray::allocate(constructor_name, new_target, proto, None, context)?;

                    // ii. If firstArgument has a [[TypedArrayName]] internal slot, then
                    if first_argument.is_typed_array() {
                        // 1. Perform ? InitializeTypedArrayFromTypedArray(O, firstArgument).
                        TypedArray::initialize_from_typed_array(&o, first_argument, context)?;
                    } else if first_argument.is_array_buffer() {
                        // iii. Else if firstArgument has an [[ArrayBufferData]] internal slot, then

                        // 1. If numberOfArgs > 1, let byteOffset be args[1]; else let byteOffset be undefined.
                        let byte_offset = args.get_or_undefined(1);

                        // 2. If numberOfArgs > 2, let length be args[2]; else let length be undefined.
                        let length = args.get_or_undefined(2);

                        // 3. Perform ? InitializeTypedArrayFromArrayBuffer(O, firstArgument, byteOffset, length).
                        TypedArray::initialize_from_array_buffer(
                            &o,
                            first_argument,
                            byte_offset,
                            length,
                            context,
                        )?;
                    } else {
                        // iv. Else,

                        // 1. Assert: Type(firstArgument) is Object and firstArgument does not have
                        // either a [[TypedArrayName]] or an [[ArrayBufferData]] internal slot.

                        // 2. Let usingIterator be ? GetMethod(firstArgument, @@iterator).

                        let first_argument_v = JsValue::from(first_argument.clone());
                        let using_iterator =
                            first_argument_v.get_method(WellKnownSymbols::replace(), context)?;

                        // 3. If usingIterator is not undefined, then
                        if !using_iterator.is_undefined() {
                            // a. Let values be ? IterableToList(firstArgument, usingIterator).
                            let values =
                                iterable_to_list(context, first_argument_v, Some(using_iterator))?;

                            // b. Perform ? InitializeTypedArrayFromList(O, values).
                            TypedArray::initialize_from_list(&o, values, context)?;
                        } else {
                            // 4. Else,

                            // a. NOTE: firstArgument is not an Iterable so assume it is already an array-like object.
                            // b. Perform ? InitializeTypedArrayFromArrayLike(O, firstArgument).
                            TypedArray::initialize_from_array_like(&o, &first_argument, context)?;
                        }
                    }

                    // v. Return O.
                    Ok(o.into())
                } else {
                    // c. Else,

                    // i. Assert: firstArgument is not an Object.
                    assert!(!first_argument.is_object(), "firstArgument was an object");

                    // ii. Let elementLength be ? ToIndex(firstArgument).
                    let element_length = first_argument.to_index(context)?;

                    // iii. Return ? AllocateTypedArray(constructorName, NewTarget, proto, elementLength).
                    Ok(TypedArray::allocate(
                        constructor_name,
                        new_target,
                        proto,
                        Some(element_length),
                        context,
                    )?
                    .into())
                }
            }
        }
    };
}

/// The JavaScript `%TypedArray%` object.
///
/// <https://tc39.es/ecma262/#sec-%typedarray%-intrinsic-object>
#[derive(Debug, Clone, Copy)]
struct TypedArray;

impl TypedArray {
    const NAME: &'static str = "TypedArray";
    const LENGTH: usize = 0;

    fn init(context: &mut Context) -> JsObject {
        let _timer = BoaProfiler::global().start_event(Self::NAME, "init");

        let get_species = FunctionBuilder::native(context, Self::get_species)
            .name("get [Symbol.species]")
            .constructable(false)
            .build();

        ConstructorBuilder::with_standard_object(
            context,
            Self::constructor,
            context.standard_objects().function_object().clone(),
        )
        .name(Self::NAME)
        .length(Self::LENGTH)
        .static_accessor(
            WellKnownSymbols::species(),
            Some(get_species),
            None,
            Attribute::CONFIGURABLE,
        )
        .property(
            "length",
            0,
            Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::PERMANENT,
        )
        // 23.2.3.29 %TypedArray%.prototype.toString ( )
        // The initial value of the %TypedArray%.prototype.toString data property is the same
        // built-in function object as the Array.prototype.toString method defined in 23.1.3.30.
        .method(Array::to_string, "toString", 0)
        .build()
    }

    /// <https://tc39.es/ecma262/#sec-%typedarray%>
    fn constructor(
        _new_target: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        // 1. Throw a TypeError exception.
        context.throw_type_error("the TypedArray constructor should never be called directly")
    }

    /// `get %TypedArray% [ @@species ]`
    ///
    /// The `%TypedArray% [ @@species ]` accessor property returns the constructor of a typed array.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-get-%typedarray%-@@species
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/@@species
    fn get_species(this: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        // 1. Return the this value.
        Ok(this.clone())
    }

    /// <https://tc39.es/ecma262/#sec-allocatetypedarraybuffer>
    fn allocate_buffer(
        indexed: &mut IntegerIndexed,
        length: usize,
        context: &mut Context,
    ) -> JsResult<()> {
        // 1. Assert: O.[[ViewedArrayBuffer]] is undefined.
        assert!(indexed.viewed_array_buffer.is_none());

        // 2. Let constructorName be the String value of O.[[TypedArrayName]].
        // 3. Let elementSize be the Element Size value specified in Table 73 for constructorName.
        let element_size = indexed.typed_array_name.element_size();

        // 4. Let byteLength be elementSize × length.
        let byte_length = element_size * length;

        // 5. Let data be ? AllocateArrayBuffer(%ArrayBuffer%, byteLength).
        let data = ArrayBuffer::allocate(
            &context
                .standard_objects()
                .array_buffer_object()
                .constructor()
                .into(),
            byte_length,
            context,
        )?;

        // 6. Set O.[[ViewedArrayBuffer]] to data.
        indexed.viewed_array_buffer = Some(data);
        // 7. Set O.[[ByteLength]] to byteLength.
        indexed.byte_length = byte_length;
        // 8. Set O.[[ByteOffset]] to 0.
        indexed.byte_offset = 0;
        // 9. Set O.[[ArrayLength]] to length.
        indexed.array_length = length;

        // 10. Return O.
        Ok(())
    }

    /// <https://tc39.es/ecma262/#sec-initializetypedarrayfromlist>
    fn initialize_from_list(
        o: &JsObject,
        values: Vec<JsValue>,
        context: &mut Context,
    ) -> JsResult<()> {
        // 1. Let len be the number of elements in values.
        let len = values.len();
        {
            let mut o = o.borrow_mut();
            let mut o_inner = o.as_typed_array_mut().expect("expected a TypedArray");

            // 2. Perform ? AllocateTypedArrayBuffer(O, len).
            TypedArray::allocate_buffer(&mut o_inner, len, context)?;
        }

        // 3. Let k be 0.
        // 4. Repeat, while k < len,
        for (k, k_value) in values.into_iter().enumerate() {
            // a. Let Pk be ! ToString(𝔽(k)).
            // b. Let kValue be the first element of values and remove that element from values.
            // c. Perform ? Set(O, Pk, kValue, true).
            o.set(k, k_value, true, context)?;
            // d. Set k to k + 1.
        }

        // 5. Assert: values is now an empty List.
        // It no longer exists.
        Ok(())
    }

    /// `AllocateTypedArray ( constructorName, newTarget, defaultProto [ , length ] )`
    ///
    /// It is used to validate and create an instance of a `TypedArray` constructor. If the `length`
    /// argument is passed, an `ArrayBuffer` of that length is also allocated and associated with the
    /// new `TypedArray` instance. `AllocateTypedArray` provides common semantics that is used by
    /// `TypedArray`.
    ///
    /// For more information, check the [spec][spec].
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-allocatetypedarray
    fn allocate<P>(
        constructor_name: TypedArrayName,
        new_target: &JsValue,
        default_proto: P,
        length: Option<usize>,
        context: &mut Context,
    ) -> JsResult<JsObject>
    where
        P: FnOnce(&StandardObjects) -> &StandardConstructor,
    {
        // 1. Let proto be ? GetPrototypeFromConstructor(newTarget, defaultProto).
        let proto = get_prototype_from_constructor(new_target, default_proto, context)?;

        let mut indexed = IntegerIndexed {
            // 3. Assert: obj.[[ViewedArrayBuffer]] is undefined.
            viewed_array_buffer: None,
            // 4. Set obj.[[TypedArrayName]] to constructorName.
            // 5. If constructorName is "BigInt64Array" or "BigUint64Array", set obj.[[ContentType]] to BigInt.
            // 6. Otherwise, set obj.[[ContentType]] to Number.
            typed_array_name: constructor_name,
            // 7. If length is not present, then
            // a. Set obj.[[ByteLength]] to 0.
            byte_length: 0,
            // b. Set obj.[[ByteOffset]] to 0.
            byte_offset: 0,
            // c. Set obj.[[ArrayLength]] to 0.
            array_length: 0,
        };

        // 8. Else,
        if let Some(length) = length {
            // a. Perform ? AllocateTypedArrayBuffer(obj, length).
            TypedArray::allocate_buffer(&mut indexed, length, context)?;
        }

        // 2. Let obj be ! IntegerIndexedObjectCreate(proto).
        let obj = IntegerIndexed::create(proto, indexed, context);

        // 9. Return obj.
        Ok(obj)
    }

    /// `23.2.5.1.2 InitializeTypedArrayFromTypedArray ( O, srcArray )`
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-initializetypedarrayfromtypedarray
    fn initialize_from_typed_array(
        o: &JsObject,
        src_array: JsObject,
        context: &mut Context,
    ) -> JsResult<()> {
        // TODO: change parameters?
        let o_obj = o.borrow();
        let src_array_obj = src_array.borrow();
        let o_array = o_obj.as_typed_array().expect("this must be a typed array");
        let src_array = src_array_obj
            .as_typed_array()
            .expect("this must be a typed array");

        // 1. Let srcData be srcArray.[[ViewedArrayBuffer]].
        // 2. If IsDetachedBuffer(srcData) is true, throw a TypeError exception.
        let src_data_obj = src_array.viewed_array_buffer().ok_or_else(|| {
            context.construct_type_error("Cannot initialize typed array from detached buffer")
        })?;
        let src_data_obj_b = src_data_obj.borrow();
        let src_data = src_data_obj_b.as_array_buffer().unwrap();

        // 3. Let constructorName be the String value of O.[[TypedArrayName]].
        // 4. Let elementType be the Element Type value in Table 73 for constructorName.
        // 10. Let elementSize be the Element Size value specified in Table 73 for constructorName.
        let constructor_name = o_array.typed_array_name();

        // 5. Let elementLength be srcArray.[[ArrayLength]].
        let element_length = src_array.array_length();

        // 6. Let srcName be the String value of srcArray.[[TypedArrayName]].
        // 7. Let srcType be the Element Type value in Table 73 for srcName.
        // 8. Let srcElementSize be the Element Size value specified in Table 73 for srcName.
        let src_name = src_array.typed_array_name();

        // 9. Let srcByteOffset be srcArray.[[ByteOffset]].
        let src_byte_offset = src_array.byte_offset();

        // 11. Let byteLength be elementSize × elementLength.
        let byte_length = constructor_name.element_size() * element_length;

        // 12. If IsSharedArrayBuffer(srcData) is false, then
        // a. Let bufferConstructor be ? SpeciesConstructor(srcData, %ArrayBuffer%).
        // TODO: Shared Array Buffer
        // 13. Else,
        // a. Let bufferConstructor be %ArrayBuffer%.
        let buffer_constructor = src_data_obj
            .species_constructor(context.global_object().get(Self::NAME, context)?, context)?;

        // 14. If elementType is the same as srcType, then
        let data = if constructor_name == src_name {
            // a. Let data be ? CloneArrayBuffer(srcData, srcByteOffset, byteLength, bufferConstructor).
            src_data.clone_array_buffer(
                src_byte_offset,
                byte_length,
                &buffer_constructor,
                context,
            )?
        // 15. Else,
        } else {
            // a. Let data be ? AllocateArrayBuffer(bufferConstructor, byteLength).
            let data_obj = ArrayBuffer::allocate(&buffer_constructor, byte_length, context)?;
            let mut data_obj_b = data_obj.borrow_mut();
            let data = data_obj_b
                .as_array_buffer_mut()
                .expect("Must be ArrayBuffer");

            // b. If IsDetachedBuffer(srcData) is true, throw a TypeError exception.
            if src_data.is_detached_buffer() {
                return Err(context
                    .construct_type_error("Cannot initialize typed array from detached buffer"));
            }

            // c. If srcArray.[[ContentType]] ≠ O.[[ContentType]], throw a TypeError exception.
            if src_name.content_type() != constructor_name.content_type() {
                return Err(context.construct_type_error(
                    "Cannot initialize typed array from different content type",
                ));
            }

            // d. Let srcByteIndex be srcByteOffset.
            let mut src_byte_index = src_byte_offset;
            // e. Let targetByteIndex be 0.
            let mut target_byte_index = 0;
            // f. Let count be elementLength.
            let mut count = element_length;
            // g. Repeat, while count > 0,
            while count > 0 {
                // i. Let value be GetValueFromBuffer(srcData, srcByteIndex, srcType, true, Unordered).
                let value = src_data.get_value_from_buffer(
                    src_byte_index,
                    src_name,
                    true,
                    SharedMemoryOrder::Unordered,
                    None,
                );

                // ii. Perform SetValueInBuffer(data, targetByteIndex, elementType, value, true, Unordered).
                data.set_value_in_buffer(
                    target_byte_index,
                    constructor_name,
                    value,
                    SharedMemoryOrder::Unordered,
                    None,
                    context,
                )?;

                // iii. Set srcByteIndex to srcByteIndex + srcElementSize.
                src_byte_index += src_name.element_size();

                // iv. Set targetByteIndex to targetByteIndex + elementSize.
                target_byte_index += constructor_name.element_size();

                // v. Set count to count - 1.
                count -= 1;
            }
            drop(data_obj_b);
            data_obj
        };

        // 16. Set O.[[ViewedArrayBuffer]] to data.
        // 17. Set O.[[ByteLength]] to byteLength.
        // 18. Set O.[[ByteOffset]] to 0.
        // 19. Set O.[[ArrayLength]] to elementLength.
        drop(o_obj);
        o.borrow_mut().data = ObjectData::integer_indexed(IntegerIndexed {
            viewed_array_buffer: Some(data),
            typed_array_name: constructor_name,
            byte_offset: 0,
            byte_length,
            array_length: element_length,
        });

        Ok(())
    }

    /// `23.2.5.1.3 InitializeTypedArrayFromArrayBuffer ( O, buffer, byteOffset, length )`
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-initializetypedarrayfromarraybuffer
    fn initialize_from_array_buffer(
        o: &JsObject,
        buffer: JsObject,
        byte_offset: &JsValue,
        length: &JsValue,
        context: &mut Context,
    ) -> JsResult<()> {
        let mut o_obj_b = o.borrow_mut();
        let o = o_obj_b
            .as_typed_array_mut()
            .expect("This must be an ArrayBuffer");
        let buffer_obj_b = buffer.borrow();
        let buffer_array = buffer_obj_b
            .as_array_buffer()
            .expect("This must be an ArrayBuffer");

        // 1. Let constructorName be the String value of O.[[TypedArrayName]].
        // 2. Let elementSize be the Element Size value specified in Table 73 for constructorName.
        let constructor_name = o.typed_array_name();

        // 3. Let offset be ? ToIndex(byteOffset).
        let offset = byte_offset.to_index(context)?;

        // 4. If offset modulo elementSize ≠ 0, throw a RangeError exception.
        if offset & constructor_name.element_size() != 0 {
            return Err(context.construct_range_error("Invalid length for typed array"));
        }

        // 6. If IsDetachedBuffer(buffer) is true, throw a TypeError exception.
        if buffer_array.is_detached_buffer() {
            return Err(
                context.construct_type_error("Cannot construct typed array from detached buffer")
            );
        }

        // 7. Let bufferByteLength be buffer.[[ArrayBufferByteLength]].
        let buffer_byte_length = buffer_array.array_buffer_byte_length();

        // 8. If length is undefined, then
        let new_byte_length = if length.is_undefined() {
            // a. If bufferByteLength modulo elementSize ≠ 0, throw a RangeError exception.
            if buffer_byte_length % constructor_name.element_size() != 0 {
                return Err(context.construct_range_error("Invalid length for typed array"));
            }

            // b. Let newByteLength be bufferByteLength - offset.
            let new_byte_length = buffer_byte_length as isize - offset as isize;

            // c. If newByteLength < 0, throw a RangeError exception.
            if new_byte_length < 0 {
                return Err(context.construct_range_error("Invalid length for typed array"));
            }

            new_byte_length as usize
        // 9. Else,
        } else {
            // 5. If length is not undefined, then
            // a. Let newLength be ? ToIndex(length).

            // a. Let newByteLength be newLength × elementSize.
            let new_byte_length = length.to_index(context)? * constructor_name.element_size();

            // b. If offset + newByteLength > bufferByteLength, throw a RangeError exception.
            if offset + new_byte_length > buffer_byte_length {
                return Err(context.construct_range_error("Invalid length for typed array"));
            }

            new_byte_length
        };

        drop(buffer_obj_b);

        // 10. Set O.[[ViewedArrayBuffer]] to buffer.
        o.viewed_array_buffer = Some(buffer);
        // 11. Set O.[[ByteLength]] to newByteLength.
        o.byte_length = new_byte_length;
        // 12. Set O.[[ByteOffset]] to offset.
        o.byte_offset = offset;
        // 13. Set O.[[ArrayLength]] to newByteLength / elementSize.
        o.array_length = new_byte_length / constructor_name.element_size();

        Ok(())
    }

    /// `23.2.5.1.5 InitializeTypedArrayFromArrayLike ( O, arrayLike )`
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-initializetypedarrayfromarraylike
    fn initialize_from_array_like(
        o: &JsObject,
        array_like: &JsObject,
        context: &mut Context,
    ) -> JsResult<()> {
        let mut o_obj_mut = o.borrow_mut();
        let o_array = o_obj_mut.as_typed_array_mut().expect("Must be typed array");

        // 1. Let len be ? LengthOfArrayLike(arrayLike).
        let len = array_like.length_of_array_like(context)?;

        // 2. Perform ? AllocateTypedArrayBuffer(O, len).
        TypedArray::allocate_buffer(o_array, len, context)?;

        drop(o_obj_mut);

        // 3. Let k be 0.
        // 4. Repeat, while k < len,
        for k in 0..len {
            // a. Let Pk be ! ToString(𝔽(k)).
            // b. Let kValue be ? Get(arrayLike, Pk).
            let k_value = array_like.get(k, context)?;

            // c. Perform ? Set(O, Pk, kValue, true).
            o.set(k, k_value, true, context)?;
        }

        Ok(())
    }
}

/// Names of all the typed arrays.
#[derive(Debug, Clone, Copy, Finalize, PartialEq)]
pub(crate) enum TypedArrayName {
    Int8Array,
    Uint8Array,
    Uint8ClampedArray,
    Int16Array,
    Uint16Array,
    Int32Array,
    Uint32Array,
    BigInt64Array,
    BigUint64Array,
    Float32Array,
    Float64Array,
}

unsafe impl Trace for TypedArrayName {
    // Safe because `TypedArrayName` is `Copy`
    empty_trace!();
}

impl TypedArrayName {
    /// Gets the element size of the given typed array name, as per the [spec].
    ///
    /// [spec]: https://tc39.es/ecma262/#table-the-typedarray-constructors
    #[inline]
    pub(crate) const fn element_size(self) -> usize {
        match self {
            Self::Int8Array | Self::Uint8Array | Self::Uint8ClampedArray => 1,
            Self::Int16Array | Self::Uint16Array => 2,
            Self::Int32Array | Self::Uint32Array | Self::Float32Array => 4,
            Self::BigInt64Array | Self::BigUint64Array | Self::Float64Array => 8,
        }
    }

    /// Gets the content type of this typed array name.
    #[inline]
    pub(crate) const fn content_type(self) -> ContentType {
        match self {
            Self::BigInt64Array | Self::BigUint64Array => ContentType::BigInt,
            _ => ContentType::Number,
        }
    }
}

typed_array!(Int8Array, "Int8Array", typed_int8_array_object);
typed_array!(Uint8Array, "Uint8Array", typed_uint8_array_object);
typed_array!(
    Uint8ClampedArray,
    "Uint8ClampedArray",
    typed_uint8clamped_array_object
);
typed_array!(Int16Array, "Int16Array", typed_int16_array_object);
typed_array!(Uint16Array, "Uint16Array", typed_uint16_array_object);
typed_array!(Int32Array, "Int32Array", typed_int32_array_object);
typed_array!(Uint32Array, "Uint32Array", typed_uint32_array_object);
typed_array!(BigInt64Array, "BigInt64Array", typed_bigint64_array_object);
typed_array!(
    BigUint64Array,
    "BigUint64Array",
    typed_biguint64_array_object
);
typed_array!(Float32Array, "Float32Array", typed_float32_array_object);
typed_array!(Float64Array, "Float64Array", typed_float64_array_object);
