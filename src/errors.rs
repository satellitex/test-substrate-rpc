/* ----------- failure boilerplate ----------- */

use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<failure::Error> for Error {
    fn from(error: failure::Error) -> Error {
        Error {
            inner: Context::new(ErrorKind::Other(error)),
        }
    }
}

use jsonrpc_core_client::RpcError;
use std::io::Error as IOError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "IO error")]
    Io,
    #[fail(display = "JSON RPC error")]
    JSONRpc(RpcError),
    #[fail(display = "JSON RPC core error")]
    JSONRpcCore(jsonrpc_core::Error),
    #[fail(display = "{}", _0)]
    Other(failure::Error),
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error {
            inner: error.context(ErrorKind::Io),
        }
    }
}

impl From<RpcError> for Error {
    fn from(error: RpcError) -> Self {
        Error {
            inner: Context::new(ErrorKind::JSONRpc(error)),
        }
    }
}

impl From<jsonrpc_core::Error> for Error {
    fn from(error: jsonrpc_core::Error) -> Self {
        Error {
            inner: Context::new(ErrorKind::JSONRpcCore(error)),
        }
    }
}
