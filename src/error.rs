use actix_web::ResponseError;



#[derive(Debug)]
pub enum Error {
    Unimplemented,
    Unknown(String),
    IoErr(std::io::Error),
    UreqErr(ureq::Error),
    // ResponseErr(String),
}

// impl ResponseError for Error {
    
// }

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Unknown(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoErr(value)
    }
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        Error::UreqErr(value)
    }
}
