use csv::Error;
use std::fmt::{Display, Formatter};

use crate::BucketParserError::UnableToReadFile;

/// Describes an error that is used to handle any issues with reading the files used for parsing
/// Errors could be flexible here, because the underlying library used wraps the errors in a `Box`
/// Any future possible parsing errors could be added here
#[derive(Debug)]
pub enum BucketParserError {
    /// The file passed in was unable to be read. The most common reasons for this error can be that
    /// the file itself is poorly formatted, causing a `Deserialize` error, or the file cannot be found
    UnableToReadFile(csv::Error),
}

impl From<csv::Error> for BucketParserError {
    fn from(ce: Error) -> Self {
        BucketParserError::UnableToReadFile(ce)
    }
}

impl Display for BucketParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnableToReadFile(ref inner) => {
                write!(f, "Unable to read the file passed in {:?}", inner)
            }
        }
    }
}

impl PartialEq for BucketParserError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UnableToReadFile(_), UnableToReadFile(_)) => true,
        }
    }
}
