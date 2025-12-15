use restate_sdk::errors::{HandlerError, TerminalError};

#[allow(dead_code)]
pub trait TerminalExt<T, E> {
    fn terminal(self) -> Result<T, HandlerError>;
    fn terminal_with_code(self, code: u16) -> Result<T, HandlerError>;
}

impl<T, E> TerminalExt<T, E> for Result<T, E>
where
    E: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
{
    fn terminal(self) -> Result<T, HandlerError> {
        self.map_err(|err| TerminalError::new(err.to_string()).into())
    }

    fn terminal_with_code(self, code: u16) -> Result<T, HandlerError> {
        self.map_err(|err| TerminalError::new_with_code(code, err.to_string()).into())
    }
}

#[macro_export]
macro_rules! terminal {
    ($msg:literal $(,)?) => {
        return Err(restate_sdk::errors::TerminalError::new($msg).into())
    };
    ($err:expr $(,)?) => {
        return Err(restate_sdk::errors::TerminalError::new($err.to_string()).into())
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(restate_sdk::errors::TerminalError::new(format!($fmt, $($arg)*)).into())
    };
}
