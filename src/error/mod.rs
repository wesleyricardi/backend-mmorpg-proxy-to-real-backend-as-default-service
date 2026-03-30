mod macros;

use std::fmt::{Display, Formatter};
use std::io;

use async_trait::async_trait;
use socket::{AppData, Responder, ServiceError};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Clone)]
pub enum Domain {
    Setting,
    Config,
    Network,
    Player,
    Runtime,
    Skill,
    Proxy,
    Protocol,
    Storage,
}

#[derive(Debug, Clone)]
pub enum InputErrorOrigin {
    InternalValidation(String),
    Validation(String),
    Conversion(String),
    Parse(String),
}

#[derive(Debug, Clone)]
pub enum InputViolationKind {
    InvalidField,
    InvalidFormat,
    MissingField,
    OutOfRange,
}

#[derive(Debug, Clone)]
pub enum ConflictKind {
    GenericConflict,
    AlreadyExists,
    Duplicate,
    Busy,
}

#[derive(Debug, Clone)]
pub enum RuleViolationKind {
    InvariantBroken,
    InvalidStateTransition,
    NotAllowed,
}

#[derive(Debug, Clone)]
pub enum IoErrorKind {
    CreateDir,
    ReadFile,
    WriteFile,
    RemoveDir,
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    Json,
    BinaryPacket,
    IntegerConversion,
    RecordItem,
}

#[derive(Debug, Clone)]
pub enum StateErrorKind {
    MissingConnection,
    MissingPlayer,
    MissingRuntimeData,
    LegacyWarmupIncomplete,
    LegacyWarmupTimeout,
    LegacyRuntimeDisconnected,
    LegacyRuntimeSnapshotTimeout,
}

#[derive(Debug, Clone)]
pub enum ProtocolErrorKind {
    InvalidPacket,
    DecodeFailed,
    EncodeFailed,
    ForwardFailed,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidInput(InputErrorOrigin),
    InputViolation(Domain, InputViolationKind),
    Conflict(Domain, ConflictKind),
    RuleViolation(Domain, RuleViolationKind),
    UnexpectedError,
    InvalidCredentials,
    EmptyData,
    DataNotFound,
    PermissionDenied,
    ReferenceNotFound,
    ReferenceInvalid,
    UniqueViolation(String),
    Io(IoErrorKind),
    Parse(ParseErrorKind),
    State(StateErrorKind),
    Protocol(ProtocolErrorKind),
}

impl ErrorKind {
    fn default_message(&self) -> String {
        match self {
            ErrorKind::InvalidInput(origin) => match origin {
                InputErrorOrigin::InternalValidation(message)
                | InputErrorOrigin::Validation(message)
                | InputErrorOrigin::Conversion(message)
                | InputErrorOrigin::Parse(message) => message.clone(),
            },
            ErrorKind::InputViolation(domain, kind) => {
                format!("input violation in {domain:?}: {kind:?}")
            }
            ErrorKind::Conflict(domain, kind) => format!("conflict in {domain:?}: {kind:?}"),
            ErrorKind::RuleViolation(domain, kind) => {
                format!("rule violation in {domain:?}: {kind:?}")
            }
            ErrorKind::UnexpectedError => "unexpected error".to_string(),
            ErrorKind::InvalidCredentials => "invalid credentials".to_string(),
            ErrorKind::EmptyData => "empty data".to_string(),
            ErrorKind::DataNotFound => "data not found".to_string(),
            ErrorKind::PermissionDenied => "permission denied".to_string(),
            ErrorKind::ReferenceNotFound => "reference not found".to_string(),
            ErrorKind::ReferenceInvalid => "invalid reference".to_string(),
            ErrorKind::UniqueViolation(field) => {
                format!("unique constraint violated for field {field}")
            }
            ErrorKind::Io(kind) => format!("io error: {kind:?}"),
            ErrorKind::Parse(kind) => format!("parse error: {kind:?}"),
            ErrorKind::State(kind) => format!("state error: {kind:?}"),
            ErrorKind::Protocol(kind) => format!("protocol error: {kind:?}"),
        }
    }
}

#[derive(Debug)]
pub struct AppError {
    pub code: ErrorKind,
    pub message: String,
    pub internal_message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl AppError {
    pub fn new(code: ErrorKind) -> Self {
        let message = code.default_message();
        Self {
            code,
            message,
            internal_message: None,
            cause: None,
        }
    }

    pub fn unique_violation(field: impl Into<String>) -> Self {
        Self::new(ErrorKind::UniqueViolation(field.into()))
    }

    pub fn unknown(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::UnexpectedError).set_internal_message(message, file!(), line!())
    }

    pub fn set_internal_message(
        mut self,
        message: impl Into<String>,
        file: &str,
        line: u32,
    ) -> Self {
        let message = message.into();
        log::error!(
            "message: {:?}\nfile: {}\nline: {}\ncause: {:#?}",
            message,
            file,
            line,
            self.cause
        );

        self.message = message.clone();
        self.internal_message = Some(message);
        self
    }

    pub fn set_cause<E>(mut self, cause: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        self.cause = Some(Box::new(cause));
        self
    }

    pub fn unexpected_error(
        message: impl Into<String>,
        file: &str,
        line: u32,
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        let message = message.into();
        log::error!(
            "message: {:?}\nfile: {}\nline: {}\ncause: {:#?}",
            message,
            file,
            line,
            cause
        );

        Self {
            code: ErrorKind::UnexpectedError,
            message: message.clone(),
            internal_message: Some(message),
            cause,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.internal_message, &self.cause) {
            (Some(message), Some(cause)) => {
                write!(
                    f,
                    "(code: {:?}, message: {}, cause: {})",
                    self.code, message, cause
                )
            }
            (Some(message), None) => write!(f, "(code: {:?}, message: {})", self.code, message),
            (None, Some(cause)) => {
                write!(
                    f,
                    "(code: {:?}, message: {}, cause: {})",
                    self.code, self.message, cause
                )
            }
            (None, None) => write!(f, "(code: {:?}, message: {})", self.code, self.message),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause
            .as_deref()
            .map(|cause| cause as &(dyn std::error::Error + 'static))
    }
}

#[async_trait]
impl ServiceError for AppError {
    async fn handler(
        &self,
        service_code: i32,
        res: Responder,
        _app_data: &AppData,
    ) -> std::result::Result<(), io::Error> {
        log::error!(
            "service handler error: client_id={}, code=0x{:X}, error={}",
            res.id,
            service_code,
            self
        );
        Ok(())
    }
}
