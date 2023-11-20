use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("io error: {0}")]
    Io(#[from] tokio::io::Error),

    #[error("utf8 conversion error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("an invalid number was received: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),

    #[error("an invalid float number was received: {0}")]
    InvalidFloatNumber(#[from] std::num::ParseFloatError),

    #[error("the time has elapsed: {0}")]
    TimeElapsed(#[from] tokio::time::error::Elapsed),

    #[error("invalid answer")]
    InvalidAnswer,

    #[error("a template parsing error occured : {0}")]
    TemplateParsing(#[from] tera::Error),

    #[error("invalid operation answer : {0}")]
    InvalidOperationAnswer(i64),
}
