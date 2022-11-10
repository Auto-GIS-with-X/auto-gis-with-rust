use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeometryError {
    #[error("too few coordinates, expected 2 or more, found {0})")]
    TooFewCoords(usize),
}
