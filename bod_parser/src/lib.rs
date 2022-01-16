#![feature(proc_macro_hygiene, decl_macro)]

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use crate::dao::Punishment;
use crate::error::BucketParserError;

pub mod dao;
mod error;

/// Parses a given `File` from the provided path
/// Returns a `Result<Vec<Punishment>, BucketParserError>` as a result of the process
///
/// # Arguments
///
/// * `file_path` - the provided file path. The file path should relative to the location of the service or absolute
fn parse_from_file_path(file_path: &str) -> Result<Vec<Punishment>, error::BucketParserError> {
    let mut rows: Vec<Punishment> = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .from_path(file_path)
        .map_err(BucketParserError::from)?;
    for res in reader.deserialize() {
        let punishment: Punishment = res.map_err(BucketParserError::from)?;
        println!("{:#?}", punishment);
        rows.push(punishment);
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use serde::ser::Error;

    use crate::error::BucketParserError;
    use crate::{parse_from_file_path, Punishment};

    fn get_expected_punishments() -> Vec<Punishment> {
        let pun1 = Punishment::new(
            "heisman".to_string(),
            "Easy",
            true,
            Some("fatchris".to_string()),
        );
        let pun2 = Punishment::new(
            "starwars".to_string(),
            "Medium",
            true,
            Some("billy".to_string()),
        );
        let pun3 = Punishment::new("ultimatewarrior".to_string(), "Hard", false, None);
        let pun4 = Punishment::new(
            "Blamo".to_string(),
            "Hard",
            true,
            Some("FatFace".to_string()),
        );
        vec![pun1, pun2, pun3, pun4]
    }

    #[test]
    fn test_parse_file() {
        let file_path = "/Users/mercanuis/GitHub/DLSBOD/bod_parser/testcsv/testGood.csv";
        let res = parse_from_file_path(file_path);
        println!("{:?}", res);
        let expected = get_expected_punishments();
        assert_eq!(Ok(expected), parse_from_file_path(file_path));
    }

    #[test]
    fn test_parse_file_deserialize_error() {
        let file_path = "bod_parser/testcsv/testParseError.csv";
        let res = parse_from_file_path(file_path);
        println!("{:?}", res);
        assert_eq!(
            Err(BucketParserError::UnableToReadFile(csv::Error::custom(
                "blah"
            ))),
            parse_from_file_path(file_path)
        );
    }

    #[test]
    fn test_parse_file_file_not_found() {
        let file_path = "bod_parser/testcsv/thisDoesntExist.csv";
        let res = parse_from_file_path(file_path);
        println!("{:?}", res);
        assert_eq!(
            Err(BucketParserError::UnableToReadFile(csv::Error::custom(
                "blah"
            ))),
            parse_from_file_path(file_path)
        );
    }
}
