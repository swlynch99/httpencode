use bytes::BufMut;

use core::ops::{Deref, DerefMut};

use crate::{HttpBuilder, Method, Result, Uri, Version};

/// Builder for HTTP requests.
pub struct RequestBuilder<'b, B: BufMut>(HttpBuilder<'b, B>);

impl<'b, B: BufMut> RequestBuilder<'b, B> {
    /// Create a new request with the provided header line.
    ///
    /// # Note
    /// If this method fails it may be partially-written into the buffer.
    /// It is necessary to reset the buffer back externally if that happens.
    pub fn new(buf: &'b mut B, method: Method, uri: Uri, version: Version) -> Result<Self> {
        Ok(Self(HttpBuilder::request(buf, method, uri, version)?))
    }
}

impl<'b, B: BufMut> Deref for RequestBuilder<'b, B> {
    type Target = HttpBuilder<'b, B>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'b, B: BufMut> DerefMut for RequestBuilder<'b, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
