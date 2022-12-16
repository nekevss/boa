//! A Rust API wrapper for Boa's `Promise` Builtin ECMAScript Object
use crate::{
    builtins::promise::{Promise, PromiseState},
    object::{JsObject, JsObjectType, ObjectData},
    Context, JsNativeError, JsResult, JsValue,
};

use boa_gc::{Finalize, Trace};
use std::ops::Deref;

/// `JsPromise` provides a wrapper for Boa's implementation of the ECMAScript `Promise` builtin object
#[derive(Debug, Clone, Trace, Finalize)]
pub struct JsPromise {
    inner: JsObject,
}

impl JsPromise {
    /// Create a new `JsPromise` object
    #[inline]
    pub fn new(executor: &JsValue, context: &mut Context) -> JsResult<Self> {
        let promise = Promise::constructor(
            context.intrinsics().constructors().promise().constructor(),
            &[executor.clone()],
            context,
        )?;

        Self { inner: promise }
    }

    /// Create a `JsPromise` from a regular expression `JsObject`
    #[inline]
    pub fn from_object(object: JsObject) -> JsResult<Self> {
        if object.borrow().is_promise() {
            Ok(Self { inner: object })
        } else {
            Err(JsNativeError::typ()
                .with_message("object is not a Promise")
                .into())
        }
    }

    /// Calls `Promise.prototype.then()`
    pub fn then(&self, on_fulfilled: JsValue, on_rejected: JsValue, context: &mut Context) -> JsResult<JsValue> {
        Promise::then(&self.inner.clone().into(), &[on_fulfilled, on_rejected], context)
    }

    /// Calls `Promise.prototype.catch()`
    pub fn catch(&self, on_rejected: JsValue, context: &mut Context) -> JsResult<JsValue> {
        Promise::catch(&self.inner.clone().into(), &[on_rejected], context)
    }

    /// Calls `Promise.prototype.finally()`
    pub fn finally(&self, on_finally: JsValue, context: &mut Context) -> JsResult<JsValue> {
        Promise::finally(&self.inner.clone().into(), &[on_finally], context)
    }
}

impl From<JsPromise> for JsObject {
    #[inline]
    fn from(o: JsPromise) -> Self {
        o.inner.clone()
    }
}

impl From<JsPromise> for JsValue {
    #[inline]
    fn from(o: JsPromise) -> Self {
        o.inner.clone().into()
    }
}

impl Deref for JsPromise {
    type Target = JsObject;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl JsObjectType for JsPromise {}

