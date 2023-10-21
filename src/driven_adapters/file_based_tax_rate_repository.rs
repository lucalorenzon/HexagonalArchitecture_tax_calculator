use std::{fs};
use crate::domain::driven_port::for_getting_tax_rate::ForGettingTaxRate;

pub struct FileBasedTaxRateRepository {
    filepath: String,
}

impl FileBasedTaxRateRepository {
    pub fn new(filepath: &str) -> Self {
        FileBasedTaxRateRepository {
            filepath: filepath.to_string(),
        }
    }

}

impl ForGettingTaxRate for FileBasedTaxRateRepository {
    fn get_tax_rate(&self) -> f32 {
        let path = self.filepath.as_str();
        fs::read_to_string(path).expect(&*format!("problems reading the file: {}", path))
            .parse().expect("file doesn't contain a float number")
    }
}