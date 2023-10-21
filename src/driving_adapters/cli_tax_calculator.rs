use std::io::stdin;
use std::str::FromStr;
use crate::domain::driving_port::for_calculating_tax::ForCalculatingTax;

pub struct CliTaxCalculator<T: ForCalculatingTax> {
    tax_calculator: T,

}

impl<T> CliTaxCalculator<T> where T:ForCalculatingTax {
    pub fn new(tax_calculator: T) -> Self {
        CliTaxCalculator {
            tax_calculator
        }
    }

    pub fn calculate_tax(&self) -> f32 {
        let amount = CliTaxCalculator::<T>::get_amount_from_user();
        self.tax_calculator.calculate_tax(amount)
    }

    fn get_amount_from_user() -> f32 {
        let mut input = String::new();
        println!("Enter the amount for which you need to calculate the tax: ");
        stdin().read_line(&mut input).expect("insert correct value");
        f32::from_str(input.trim()).expect("the value isn't a decimal number")
    }
}