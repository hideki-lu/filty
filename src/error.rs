use image::error::ImageError;

#[derive(Debug)]
pub enum Error {
    IndexOutOfBounds,
    ImageError(ImageError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<ImageError> for Error {
    fn from(err: ImageError) -> Self {
        Self::ImageError(err)
    }
}
