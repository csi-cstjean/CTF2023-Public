pub use crate::error::ServerError;

pub type ServerResult<T> = core::result::Result<T, ServerError>;
