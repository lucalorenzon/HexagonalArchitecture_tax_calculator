
mod domain;
mod driven_adapters;
mod driving_adapters;

use std::env;
use crate::domain::tax_calculator::TaxCalculator;
use crate::driven_adapters::file_based_tax_rate_repository::FileBasedTaxRateRepository;
use crate::driven_adapters::tax_rate_repository::TaxRateRepository;
use crate::driving_adapters::cli_tax_calculator::CliTaxCalculator;

fn main() {
    scenario_1();
    scenario_2();
}

fn scenario_1() {
    println!("Memory based fixed tax repo");
    let tax_rate_repo = TaxRateRepository::new(1f32);
    let tax_calculator = TaxCalculator::new(tax_rate_repo);
    let cli_tax_calculator = CliTaxCalculator::new(tax_calculator);
    let tax_amount = cli_tax_calculator.calculate_tax();

    println!("tax_amount {}", tax_amount);
}

fn scenario_2() {
    println!("File based fixed tax repo");
    println!("Current directory: {:?}", env::current_dir().expect("cannot read current working directory"));
    let tax_rate_repo = FileBasedTaxRateRepository::new("./src/resources/tax_rate.txt");
    let tax_calculator = TaxCalculator::new(tax_rate_repo);
    let cli_tax_calculator = CliTaxCalculator::new(tax_calculator);
    let tax_amount = cli_tax_calculator.calculate_tax();

    println!("tax_amount {}", tax_amount);
}
