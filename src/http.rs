use bytes::{Buf, BufMut};

use crate::util::{
    lookup_status_line, validate_header_field, validate_header_name, write_request_line,
    write_status_line,
};
use crate::{Error, Method, Result, Status, Uri, Version};

/// Builder for HTTP requests.
pub struct HttpBuilder<'b, B: BufMut> {
    buf: &'b mut B,
}

impl<'b, B: BufMut> HttpBuilder<'b, B> {
    /// Create a new request with the provided header line.
    ///
    /// # Note
    /// If this method fails it may be partially-written into the buffer.
    /// It is necessary to reset the buffer back externally if that happens.
    pub fn request(buf: &'b mut B, method: Method, uri: Uri, version: Version) -> Result<Self> {
        write_request_line(buf, method, uri, version)?;

        Ok(Self { buf })
    }

    /// Create a new response from the provided status line.
    pub fn response(buf: &'b mut B, version: Version, status: Status) -> Result<Self> {
        Self::response_with_reason(buf, version, status, lookup_status_line(status))
    }

    /// Create a new response from the provided status line.
    pub fn response_with_reason(
        buf: &'b mut B,
        version: Version,
        status: Status,
        reason: &str,
    ) -> Result<Self> {
        write_status_line(buf, version, status, reason)?;

        Ok(Self { buf })
    }

    /// Add a new header to the request. This method does not check
    /// whether the given header has already been specified and whether
    /// it is valid to do so.
    ///
    /// # Note
    /// This method is atomic - if it fails then nothing will be written
    /// to the buffer.
    pub fn header(&mut self, key: impl AsRef<[u8]>, val: impl AsRef<[u8]>) -> Result<&mut Self> {
        let key = key.as_ref();
        let val = val.as_ref();

        if !validate_header_name(key) {
            return Err(Error::InvalidHeaderKey);
        }
        if !validate_header_field(val) {
            return Err(Error::InvalidHeaderValue);
        }

        unsafe { self.header_unchecked(key, val) }
    }

    /// Add a new header to the request without checking to ensure that it's
    /// valid.
    ///
    /// # Note
    /// This method is atomic - if it fails then nothing will be written
    /// to the buffer.
    pub unsafe fn header_unchecked(
        &mut self,
        key: impl AsRef<[u8]>,
        val: impl AsRef<[u8]>,
    ) -> Result<&mut Self> {
        let key = key.as_ref();
        let val = val.as_ref();

        let est_required = key.len() + val.len() + b": \r\n".len();
        if est_required < self.buf.remaining_mut() {
            return Err(Error::OutOfBuffer);
        }

        self.buf.put_slice(key);
        self.buf.put_slice(b": ");
        self.buf.put_slice(val);
        self.buf.put_slice(b"\r\n");

        Ok(self)
    }

    /// Complete the HTTP header and return the underlying buffer.
    pub fn finish(self) -> Result<&'b mut B> {
        if self.buf.remaining_mut() < b"\r\n".len() {
            return Err(Error::OutOfBuffer);
        }

        self.buf.put_slice(b"\r\n");

        Ok(self.buf)
    }

    /// Complete the HTTP header and return the underlying buffer.
    pub fn body<I: Buf>(self, buf: &mut I) -> Result<&'b mut B> {
        let len = buf.remaining();

        if len + b"\r\n".len() < self.buf.remaining_mut() {
            return Err(Error::OutOfBuffer);
        }

        self.buf.put_slice(b"\r\n");

        while buf.has_remaining() {
            let bytes = buf.bytes();
            self.buf.put_slice(bytes);

            let len = bytes.len();
            buf.advance(len);
        }

        Ok(self.buf)
    }

    /// Get the underlying buffer for this request object.
    pub fn into_buf(self) -> &'b mut B {
        self.buf
    }

    /// Create a request from the underlying buffer.
    ///
    /// # Safety
    /// This function is unsafe since you can use it to create a syntactically
    /// invalid request.
    pub unsafe fn from_buf(buf: &'b mut B) -> Self {
        Self { buf }
    }

    /// Get the number of remaining bytes within the underlying buffer.
    pub fn remaining(&self) -> usize {
        self.buf.remaining_mut()
    }
}
