use std::fmt;

///A Customerror which implements selfmade error types and a wrapper around existing error types.
pub enum CustomError {
    IoError(std::io::Error),
    ImageError(image::ImageError),
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    UnfittingArray,
    TimerError,
    ThreadPanic,
    InvalidArgument,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::ImageError(ref e) => write!(f, "{}", e),
            CustomError::ParseIntError(ref e) => write!(f, "{}", e),
            CustomError::ParseFloatError(ref e) => write!(f, "{}", e),
            CustomError::UnfittingArray => {
                write!(f, "The Array size is to small for the specified bounds.")
            }
            CustomError::TimerError => {
                write!(f, "The unsafe call of Systemcall clock_gettime threw an error")
            }
            CustomError::ThreadPanic => write!(f, "A Thread paniced"),
            CustomError::InvalidArgument => write!(
                f,
                "Invalid Argument. Rerun with --help for more Information."
            ),
        }
    }
}

impl fmt::Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::ImageError(ref e) => write!(f, "{}", e),
            CustomError::ParseIntError(ref e) => write!(f, "{}", e),
            CustomError::ParseFloatError(ref e) => write!(f, "{}", e),
            CustomError::UnfittingArray => {
                write!(f, "The Array size is to small for the specified bounds.")
            }
            CustomError::TimerError => {
                write!(f, "The unsafe call of Systemcall clock_gettime threw an error")
            }
            CustomError::ThreadPanic => write!(f, "A Thread paniced"),
            CustomError::InvalidArgument => write!(
                f,
                "Invalid Argument. Rerun with --help for more Information."
            ),
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::IoError(error)
    }
}

impl From<image::ImageError> for CustomError {
    fn from(error: image::ImageError) -> Self {
        CustomError::ImageError(error)
    }
}

impl From<std::num::ParseIntError> for CustomError {
    fn from(error: std::num::ParseIntError) -> Self {
        CustomError::ParseIntError(error)
    }
}

impl From<std::num::ParseFloatError> for CustomError {
    fn from(error: std::num::ParseFloatError) -> Self {
        CustomError::ParseFloatError(error)
    }
}

impl From<std::boxed::Box<dyn std::any::Any + std::marker::Send>> for CustomError {
    fn from(_error: std::boxed::Box<dyn std::any::Any + std::marker::Send>) -> Self {
        CustomError::ThreadPanic
    }
}
