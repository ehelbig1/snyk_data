use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Request failed")]
    RequestError,

    #[error("Failed to parse response data")]
    ParseError,

    #[error("Missing SNYK_API_KEY environment variable")]
    EnvironmentError,
}
