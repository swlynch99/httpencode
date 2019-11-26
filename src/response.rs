use bytes::BufMut;

use std::ops::{Deref, DerefMut};

use crate::{HttpBuilder, Result, Status, Version};

/// Builder for HTTP requests.
pub struct ResponseBuilder<'b, B: BufMut>(HttpBuilder<'b, B>);

impl<'b, B: BufMut> ResponseBuilder<'b, B> {
    /// Create a new request with the provided status line.
    ///
    /// # Note
    /// If this method fails it may be partially-written into the buffer.
    /// It is necessary to reset the buffer back externally if that happens.
    pub fn new(buf: &'b mut B, version: Version, status: Status) -> Result<Self> {
        Ok(Self(HttpBuilder::response(buf, version, status)?))
    }
}

impl<'b, B: BufMut> Deref for ResponseBuilder<'b, B> {
    type Target = HttpBuilder<'b, B>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'b, B: BufMut> DerefMut for ResponseBuilder<'b, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
