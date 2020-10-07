use err_derive::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "I/O error: {}")]
    Io(#[error(source)] std::io::Error),
    #[error(display = "HTTP client error: {}")]
    Request(#[error(source)] reqwest::Error),
    #[error(display = "TOML deserialization error: {}")]
    Deserialize(#[error(source)] toml::de::Error),
}
