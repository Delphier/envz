mod env;
pub use env::Environment;

pub mod registry;
pub mod var;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    JoinPaths(#[from] std::env::JoinPathsError),
    #[error(transparent)]
    WindowsRegistry(#[from] windows_result::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
