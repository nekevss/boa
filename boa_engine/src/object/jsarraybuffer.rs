// This module create an API wrapper for the Boa ArrayBuffer object

use crate::{
    builtins::array_buffer::{create_byte_data_block, ArrayBuffer},
    object::{JsObject, JsObjectType, ObjectData},
    Context, JsResult, JsValue,
};

use boa_gc::{Finalize, Trace};
use std::ops::Deref;

#[derive(Debug, Clone, Finalize, Trace)]
pub struct JsArrayBuffer {
    inner: JsObject,
}

impl JsArrayBuffer {
    #[inline]
    pub fn new(byte_length: &JsValue, context: &mut Context) -> JsResult<Self> {
        let byte_usize = byte_length.to_index(context)?;

        let array_buffer = Self::create_and_allocate_buffer(byte_usize, context)?;

        Ok(Self {
            inner: array_buffer,
        })
    }

    #[inline]
    pub fn from_object(object: JsObject, context: &mut Context) -> JsResult<Self> {
        if object.borrow().is_array_buffer() {
            Ok(Self { inner: object })
        } else {
            context.throw_type_error("Object is not an ArrayBuffer")
        }
    }

    #[inline]
    fn create_and_allocate_buffer(byte_length: usize, context: &mut Context) -> JsResult<JsObject> {
        let prototype = context
            .intrinsics()
            .constructors()
            .array_buffer()
            .prototype();

        let block = create_byte_data_block(byte_length, context)?;

        Ok(JsObject::from_proto_and_data(
            prototype,
            ObjectData::array_buffer(ArrayBuffer {
                array_buffer_data: Some(block),
                array_buffer_byte_length: byte_length,
                array_buffer_detach_key: JsValue::Undefined,
            }),
        ))
    }

    #[inline]
    pub fn byte_length(&self) -> usize {
        let inner_data = &self
            .inner
            .borrow()
            .as_array_buffer()
            .expect("object must be ArrayBuffer")
            .clone();
        inner_data.array_buffer_byte_length
    }

    #[inline]
    pub fn is_detached_buffer(&self) -> bool {
        let inner_data = &self
            .inner
            .borrow()
            .as_array_buffer()
            .expect("Must be an ArrayBuffer")
            .clone();
        inner_data.array_buffer_data.is_none()
    }

    #[inline]
    pub fn slice<T>(&self, start: usize, end: usize, context: &mut Context) -> JsResult<JsValue> {
        if self.is_detached_buffer() {
            return context.throw_type_error("ArrayBuffer is detached.");
        }
        ArrayBuffer::slice(
            &self.inner.clone().into(),
            &[start.into(), end.into()],
            context,
        )
    }

    #[inline]
    pub fn clone_buffer_data(&self, context: &mut Context) -> JsResult<Vec<u8>> {
        if self.is_detached_buffer() {
            return context.throw_type_error("ArrayBuffer is detached. No data to clone.");
        }
        let inner_data = &self
            .inner
            .borrow()
            .as_array_buffer()
            .expect("Inner must be an ArrayBuffer")
            .clone();
        Ok(inner_data
            .array_buffer_data
            .clone()
            .expect("array_buffer_data must exist"))
    }

    #[inline]
    pub fn take_buffer_data(&self, context: &mut Context) -> JsResult<Vec<u8>> {
        if self.is_detached_buffer() {
            return context.throw_type_error("ArrayBuffer is detached. No data to take.");
        }
        let inner_data = &mut self
            .inner
            .borrow_mut()
            .as_array_buffer()
            .expect("Inner must be an ArrayBuffer")
            .clone();
        Ok(inner_data
            .array_buffer_data
            .take()
            .expect("inner_data must exist"))
    }
}

impl From<JsArrayBuffer> for JsObject {
    #[inline]
    fn from(o: JsArrayBuffer) -> Self {
        o.inner.clone()
    }
}

impl From<JsArrayBuffer> for JsValue {
    #[inline]
    fn from(o: JsArrayBuffer) -> Self {
        o.inner.clone().into()
    }
}

impl Deref for JsArrayBuffer {
    type Target = JsObject;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl JsObjectType for JsArrayBuffer {}
