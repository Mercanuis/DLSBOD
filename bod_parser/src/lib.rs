#![feature(proc_macro_hygiene, decl_macro)]

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use crate::dao::Punishment;
use std::error::Error;

mod dao;

type CsvParseResult = Result<(), Box<dyn Error>>;

fn parse_from_file_path(file_path: &str) -> CsvParseResult {
    let mut reader = csv::ReaderBuilder::new().from_path(file_path)?;
    for res in reader.deserialize() {
        let punishment: Punishment = res?;
        println!("{:#?}", punishment)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_from_file_path;

    #[test]
    fn test_parse_file() {
        let file_path = "/Users/mercanuis/GitHub/DLSBOD/bod_parser/testcsv/test1.csv";
        let res = parse_from_file_path(file_path);
        println!("{:?}", res);
        assert!(res.is_ok());
        //assert_eq!(Ok(()), parse_from_file_path(file_path));
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
