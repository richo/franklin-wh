use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Issue with the http library or transport")]
    Transport(#[from] reqwest::Error),
    #[error("Application error from the Franklin API")]
    BadRequest {
        code: i32,
        message: String,
    },
}
