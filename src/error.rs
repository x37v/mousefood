use core::{error, fmt};

#[derive(Debug)]
pub struct DrawError;

impl error::Error for DrawError {}

impl fmt::Display for DrawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "writing to display failed")
    }
}
