//! Zero-allocation HTTP encoding.

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

#[cfg(feature = "std")]
#[macro_use]
extern crate thiserror;

mod http;
mod util;
mod traits;

pub use self::http::HttpBuilder;
pub use self::traits::{HeaderValue, OutOfBufferError};

#[cfg(test)]
mod tests;

/// HTTP method.
#[derive(Copy, Clone, Debug)]
pub enum Method<'a> {
    Options,
    Get,
    Head,
    Post,
    Put,
    Patch,
    Delete,
    Trace,
    Connect,
    Custom(&'a str),
}

/// HTTP version.
#[derive(Copy, Clone, Debug)]
pub enum Version<'a> {
    Http10,
    Http11,
    Custom(&'a str),
}

#[derive(Copy, Clone)]
enum UriData<'a> {
    Escaped(&'a [u8]),
    Unescaped(&'a [u8]),
}

/// HTTP resource identifier.
#[derive(Copy, Clone)]
pub struct Uri<'a> {
    data: UriData<'a>,
}

/// HTTP Status code.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Status {
    code: u16,
}

#[derive(Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "std", derive(Error))]
pub enum Error {
    /// Ran out of buffer space
    #[cfg_attr(feature = "std", error("Out of buffer space"))]
    OutOfBuffer,
    /// A custom method contained invalid characters
    #[cfg_attr(feature = "std", error("Invalid HTTP method"))]
    InvalidMethod,
    /// The URI contained invalid characters
    #[cfg_attr(feature = "std", error("Invalid HTTP URI"))]
    InvalidUri,
    /// A custom version contains invalid characters
    #[cfg_attr(feature = "std", error("Invalid HTTP version"))]
    InvalidVersion,
    /// A header key contained invalid characters
    #[cfg_attr(feature = "std", error("Invalid header key"))]
    InvalidHeaderKey,
    /// A header value contained invalid characters
    #[cfg_attr(feature = "std", error("Invalid header value"))]
    InvalidHeaderValue,
}

type Result<T> = core::result::Result<T, Error>;

impl Status {
    /// Create a new status from the provided status code.
    ///
    /// # Panics
    /// Panics if the code is greater than `1000`.
    pub fn new(code: u16) -> Self {
        assert!(code < 1000);

        Self { code }
    }
}

impl<'a> Uri<'a> {
    pub fn new(uri: &'a [u8]) -> Self {
        Self {
            data: UriData::Unescaped(uri.as_ref()),
        }
    }

    pub unsafe fn escaped_unchecked(uri: &'a [u8]) -> Self {
        Self {
            data: UriData::Escaped(uri),
        }
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        match self.data {
            UriData::Escaped(s) => s,
            UriData::Unescaped(s) => s,
        }
    }
}

macro_rules! statuses {
    {
        $( $status:expr => $name:ident; )*
    } => {
        impl Status {
            $(
                pub const $name: Status = Status { code: $status };
            )*
        }
    }
}

statuses! {
    100 => CONTINUE;
    101 => SWITCHING_PROTOCOLS;
    102 => PROCESSING;
    103 => EARLY_HINTS;

    200 => OK;
    201 => CREATED;
    202 => ACCEPTED;
    203 => NON_AUTHORATATIVE_INFORMATION;
    204 => NO_CONTENT;
    205 => RESET_CONTENT;
    206 => PARTIAL_CONTENT;
    207 => MULTI_STATUS;
    208 => ALREADY_REPORTED;
    226 => IM_USED;

    300 => MULTIPLE_CHOICES;
    301 => MOVED_PERMANENTLY;
    302 => FOUND;
    303 => SEE_OTHER;
    304 => NOT_MODIFIED;
    305 => USE_PROXY;
    // 306 is obsolete
    307 => TEMPORARY_REDIRECT;
    308 => PERMANENT_REDIRECT;

    400 => BAD_REQUEST;
    401 => UNAUTHORIZED;
    402 => PAYMENT_REQUIRED;
    403 => FORBIDDEN;
    404 => NOT_FOUND;
    405 => METHOD_NOT_ALLOWED;
    406 => NOT_ACCEPTABLE;
    407 => PROXY_AUTHENTICATION_TIMEOUT;
    408 => REQUEST_TIMEOUT;
    409 => CONFLICT;
    410 => GONE;
    411 => LENGTH_REQUIRED;
    412 => PRECONDITION_FAILED;
    413 => REQUEST_ENTITY_TOO_LARGE;
    414 => REQUEST_URI_TOO_LARGE;
    415 => UNSUPPORTED_MEDIA_TYPE;
    416 => REQUESTED_RANGE_NOT_SATISFIABLE;
    417 => EXPECTATION_FAILED;
    418 => IM_A_TEAPOT;
    421 => MISDIRECTED_REQUEST;
    422 => UNPROCESSABLE_ENTITY;
    423 => LOCKED;
    424 => FAILED_DEPENDENCY;
    425 => TOO_EARLY;
    426 => UPGRADE_REQUIRED;
    428 => PRECONDITION_REQUIRED;
    429 => TOO_MANY_REQUESTS;
    451 => UNAVAILABLE_FOR_LEGAL_REASONS;

    500 => INTERNAL_SERVER_ERROR;
    501 => NOT_IMPLEMENTED;
    502 => BAD_GATEWAY;
    503 => SERVICE_UNAVAILABLE;
    504 => GATEWAY_TIME_OUT;
    505 => HTTP_VERSION_NOT_SUPPORTED;
    506 => VARIANT_ALSO_NEGOTIATES;
    507 => INSUFFICIENT_STORAGE;
    508 => LOOP_DETECTED;
}

impl From<OutOfBufferError> for Error {
    fn from(_: OutOfBufferError) -> Self {
        Self::OutOfBuffer
    }
}
