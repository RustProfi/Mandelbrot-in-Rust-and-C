extern crate image;
use std::fmt;

///A Customerror which implements selfmade error types and a wrapper around existing error types.
pub enum CustomError {
    UnfittingArray,
    IoError(std::io::Error),
    ImageError(image::ImageError),
    TimerError,
    ThreadPanic,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::UnfittingArray => write!(f, "Array size is to small for bounds."),
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::ImageError(ref e) => write!(f, "{}", e),
            CustomError::TimerError => write!(f, "Unsafe C Timer threw an error"),
            CustomError::ThreadPanic => write!(f, "Thread paniced"),
        }
    }
}

impl fmt::Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CustomError::UnfittingArray => write!(f, "Array size is to small for bounds."),
            CustomError::IoError(ref e) => write!(f, "{}", e),
            CustomError::ImageError(ref e) => write!(f, "{}", e),
            CustomError::TimerError => write!(f, "Unsafe C Timer threw an error"),
            CustomError::ThreadPanic => write!(f, "Thread paniced"),
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
