use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct LispError {
    explanation: Option<String>,
    err_type: ErrorType,
}

impl Error for LispError {}

impl Display for LispError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        if self.explanation.is_some() {
            write!(
                f,
                "{} => {}",
                self.err_type,
                self.explanation.clone().unwrap()
            )
        } else {
            write!(f, "{}", self.err_type)
        }
    }
}

impl LispError {
    pub fn parsing_error(line: usize, file_name: String) -> Self {
        Self {
            explanation: None,
            err_type: ErrorType::ParsingError {
                line,
                file: file_name,
            },
        }
    }

    pub fn runtime_error<T: ToString>(obj: T) -> Self {
        Self {
            explanation: None,
            err_type: ErrorType::RuntimeError(obj.to_string()),
        }
    }

    pub fn add_reason<T: ToString>(mut self, reason: T) -> Self {
        self.explanation = Some(reason.to_string());
        self
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum ErrorType {
    ParsingError { line: usize, file: String },
    RuntimeError(String),
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "{}",
            match self {
                ErrorType::ParsingError { line, file } => format!("{}: {}", line, file),
                ErrorType::RuntimeError(err) => err.to_string(),
            }
        )
    }
}
