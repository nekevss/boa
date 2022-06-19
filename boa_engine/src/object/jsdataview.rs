// This module creates an API wrapper for the Boa DataView object

use crate::{
    builtins::DataView,
    object::{JsObject, ObjectData, JsObjectType, JsArrayBuffer},
    Context, JsValue, JsResult,
};

use boa_gc::{Finalize, Trace};
use std::ops::Deref;

#[derive(Debug, Clone, Trace, Finalize)]
pub struct JsDataView {
    inner: JsObject
}

impl JsDataView {
    #[inline]
    pub fn from_js_array_buffer(buffer: &JsArrayBuffer, offset: Option<JsValue>, byte_length: Option<JsValue>, context: &mut Context) -> JsResult<Self> {

        let prototype = context.intrinsics().constructors().data_view().prototype();

        // TODO: Build out the below
        let requested_bytes = match byte_length {
            Some(bytes) => bytes,
            None => JsValue::undefined(),
        };
        let requested_offset = match offset {
            Some(offset_amount) => offset_amount.to_index(context)?,
            None => 0,
        };

        if buffer.is_detached_buffer() {
            return context.throw_type_error("ArrayBuffer is detached")
        }

        let buffer_length = buffer.byte_length();

        if requested_offset > buffer_length {
            return context.throw_range_error("Requested offset is outside the bounds of the buffer")
        }

        let view_bytes = if requested_bytes.is_undefined() {
            buffer_length - requested_offset
        } else {
            let view_bytes = requested_bytes.to_index(context)?;

            if requested_offset + view_bytes > buffer_length {
                return context.throw_range_error("Invalid data view length")
            }

            view_bytes
        };

        Ok(Self { inner: JsObject::from_proto_and_data(prototype, ObjectData::data_view( DataView {
            viewed_array_buffer: buffer.clone().into(),
            byte_length: view_bytes,
            byte_offset: requested_offset
        }))})
    }

    #[inline]
    pub fn from_object(object: JsObject, context: &mut Context) -> JsResult<Self> {
        if object.borrow().is_data_view() {
            Ok(Self {inner: object})
        } else {
            context.throw_type_error("object is not a DataView")
        }
    }

    #[inline]
    pub fn buffer(&self, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_buffer(&self.inner.clone().into(), &[], context)
    }

    #[inline]
    pub fn byte_length(&self, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_byte_length(&self.inner.clone().into(), &[], context)
    }

    #[inline]
    pub fn byte_offset(&self, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_byte_offset(&self.inner.clone().into(), &[], context)
    }

    #[inline]
    pub fn get_big_int64(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_big_int64(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn get_big_uint64(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_big_uint64(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn get_float32(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_float32(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn get_float64(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_float64(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn get_int8(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_int8(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn get_int16(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_int16(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn get_int32(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_int32(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }
    
    #[inline]
    pub fn get_uint8(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_uint8(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn get_unit16(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_uint16(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn get_unit32(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::get_uint32(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn set_big_int64(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::set_big_int64(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn set_big_uint64(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::set_big_uint64(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn set_float32(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::set_float32(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn set_float64(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::set_float64(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn set_int8(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::set_int8(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn set_int16(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::set_int16(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn set_int32(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::set_int32(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }
    
    #[inline]
    pub fn set_uint8(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::set_uint8(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn set_unit16(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::set_uint16(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }

    #[inline]
    pub fn set_unit32(&self, byte_offset: usize, is_little_edian: bool, context: &mut Context) -> JsResult<JsValue> {
        DataView::set_uint32(&self.inner.clone().into(), &[byte_offset.into(), is_little_edian.into()], context)
    }
}

impl From<JsDataView> for JsObject {
    #[inline]
    fn from(o: JsDataView) -> Self {
        o.inner.clone()
    }
}

impl From<JsDataView> for JsValue {
    #[inline]
    fn from(o: JsDataView) -> Self {
        o.inner.clone().into()
    }
}

impl Deref for JsDataView {
    type Target = JsObject;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl JsObjectType for JsDataView {}
