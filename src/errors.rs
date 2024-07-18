use std::fmt;

#[derive(Debug, Clone)]
pub struct MinPercentageError;

impl fmt::Display for MinPercentageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Percentage must be between 0 and 1")
    }
}
