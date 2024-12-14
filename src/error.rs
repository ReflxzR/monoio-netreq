use http::Error as HttpError;
use http::header::InvalidHeaderValue;
use monoio_transports::{FromUriError, TransportError};
#[cfg(not(feature = "hyper-tls"))]
use monoio_http::common::error::HttpError as MonoioHttpError;
#[cfg(any(feature = "hyper", feature = "pool-hyper", feature = "hyper-tls"))]
use monoio_transports::connectors::pollio::PollConnectError;
#[cfg(any(feature = "hyper", feature = "pool-hyper", feature = "hyper-tls"))]
use monoio_transports::http::hyper::HyperError;
use serde_json::Error as SerdeError;
use thiserror::Error as ThisError;

#[cfg(not(feature = "hyper-tls"))]
pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0:?}")]
    InvalidHeaderValue(InvalidHeaderValue),
    #[error("error building http request: {0:?}")]
    HttpRequestBuilder(HttpError),
    #[error("http request version and http protocol does not match: {0:?}")]
    HttpVersionMismatch(String),
    #[error("error making pool key from uri: {0:?}")]
    UriKeyError(FromUriError),
    #[error("http transport error requesting a connection: {0:?}")]
    HttpTransportError(TransportError),
    #[cfg(any(feature = "hyper", feature = "pool-hyper", feature = "hyper-tls"))]
    #[error("hyper transport error requesting a connection: {0:?}")]
    HyperTransportError(HyperError<PollConnectError<std::io::Error>>),
    #[cfg(not(feature = "hyper-tls"))]
    #[error("{0:?}")]
    HttpResponseError(MonoioHttpError),
    #[cfg(any(feature = "hyper", feature = "pool-hyper", feature = "hyper-tls"))]
    #[error("{0:?}")]
    HyperResponseError(hyper::Error),
    #[error("{0:?}")]
    BytesError(String),
    #[error("serde body deserialize error: {0:?}")]
    SerdeDeserializeError(SerdeError),
    #[error("Hyper Connector was not initialized")]
    ConnectorNotInitialized,
}
