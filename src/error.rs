use std::error::Error as StdError;
use std::fmt;

type Cause = Box<StdError + Send + Sync>;

#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorImpl>,
}

#[derive(Debug)]
struct ErrorImpl {
    kind: Kind,
    cause: Option<Cause>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Kind {
    InvalidCredential,
}

impl Error {
    pub(crate) fn new(kind: Kind, cause: Option<Cause>) -> Error {
        Error {
            inner: Box::new(ErrorImpl {
                kind,
                cause,
            }),
        }
    }

    pub(crate) fn kind(&self) -> &Kind {
        &self.inner.kind
    }

    pub(crate) fn new_invalid_credential() -> Error {
        Error::new(Kind::InvalidCredential, None)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.inner.kind {
            Kind::InvalidCredential => "invalid credential",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        self.inner.cause.as_ref().map(|cause| &**cause as &StdError)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref cause) = self.inner.cause {
            write!(f, "{}: {}", self.description(), cause)
        } else {
            f.write_str(self.description())
        }
    }
}
