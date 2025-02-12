pub type AppError = anyhow::Error;

pub type AppResult<T> = anyhow::Result<T>;

// #[derive(Error, Debug)]
// pub enum AppError {
//     #[error("internal error: {0}")]
//     Internal(#[from] Box<dyn std::error::Error + Send + Sync>),
//     #[error("request error: {0}")]
//     Request(#[from] RequestError),
//     #[error("status error: {0}")]
//     Status(#[from] Status),
//     #[error("broker error: {0}")]
//     Broker(#[from] BrokerError),
//     #[error("cache error: {0}")]
//     Cache(#[from] CacheError),
// }

// macro_rules! impl_internal_errors {
//     ( $( $type:ty ),* $(,)? ) => {
//         $(
//         impl From<$type> for AppError {
//             fn from(err: $type) -> Self {
//                 AppError::Internal(Box::new(err))
//             }
//         }
//         )*
//     };
// }
// impl_internal_errors!(
//     config::ConfigError,
//     transport::Error,
//     RegistryError,
//     PostgresError,
//     PostgresPoolError,
// );

// macro_rules! impl_request_errors {
//     ( $( $type:ty ),* $(,)? ) => {
//         $(
//         impl From<$type> for AppError {
//             fn from(err: $type) -> Self {
//                 RequestError::from(err).into()
//             }
//         }
//         )*
//     };
// }
// impl_request_errors!(
//     PlatformError,
//     ModuleError,
//     NodeError,
//     GraphError,
//     EditorError,
//     EngineError,
//     CommonError,
// );

// impl From<AppError> for Status {
//     fn from(err: AppError) -> Self {
//         match err {
//             AppError::Request(err) => err.into(),
//             AppError::Status(status) => status,
//             _ => {
//                 error!("internal service error: {}", err);
//                 Status::internal(Code::Internal.description())
//             }
//         }
//     }
// }

// pub type AppResult<T> = Result<T, AppError>;
