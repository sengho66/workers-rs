use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomError {
    message: String,
    status: u16,
}

impl CustomError {
    pub fn new(message: String, status: u16) -> Self {
        Self { message, status }
    }
}

impl From<CustomError> for worker::Error {
    fn from(err: CustomError) -> Self {
        Self::Json((serde_json::to_value(&err).unwrap().to_string(), err.status))
    }
}

impl From<worker::Error> for CustomError {
    fn from(err: worker::Error) -> Self {
        if let worker::Error::Json((message, status)) = err {
            return Self { message, status };
        }

        Self {
            message: err.to_string(),
            status: 500,
        }
    }
}
