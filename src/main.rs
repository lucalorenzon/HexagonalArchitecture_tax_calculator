
mod domain;
mod driven_adapters;
mod driving_adapters;

use crate::domain::tax_calculator::TaxCalculator;
use crate::driven_adapters::tax_rate_repository::TaxRateRepository;
use crate::driving_adapters::cli_tax_calculator::CliTaxCalculator;

fn main() {

    let tax_rate_repo = TaxRateRepository::new(1f32);
    let tax_calculator = TaxCalculator::new(tax_rate_repo);
    let cli_tax_calculator = CliTaxCalculator::new(tax_calculator);
    let tax_amount = cli_tax_calculator.calculate_tax();

    println!("tax_amount {}", tax_amount);
}
