use crate::domain::driven_port::for_getting_tax_rate::ForGettingTaxRate;
use crate::domain::driving_port::for_calculating_tax::ForCalculatingTax;

pub struct TaxCalculator<T: ForGettingTaxRate> {
    getting_tax_rate: T,
}

impl<T> TaxCalculator<T> where T: ForGettingTaxRate {
    pub fn new(getting_tax_rate: T) -> Self {
        Self {
            getting_tax_rate,
        }
    }
}

impl <T: ForGettingTaxRate> ForCalculatingTax for TaxCalculator<T> {
    fn calculate_tax(&self, amount: f32) -> f32 {
        amount * self.getting_tax_rate.get_tax_rate() / 100f32
    }
}


#[cfg(test)]
mod tests {
    use crate::domain::driven_port::for_getting_tax_rate::ForGettingTaxRate;
    use crate::domain::driving_port::for_calculating_tax::ForCalculatingTax;
    use crate::domain::tax_calculator::TaxCalculator;

    #[test]
    fn should_return_the_correct_amount_of_tax_to_pay() {
        let mock_getting_tax_rate = mock_for_getting_tax_rate(1f32);
        let tax_calculator = TaxCalculator::new(mock_getting_tax_rate);
        let result = tax_calculator.calculate_tax(1000f32);
        assert_eq!(result, 10f32);
    }

    fn mock_for_getting_tax_rate(tax_rate: f32) -> impl ForGettingTaxRate {
        struct MockedGettingTaxRate {
            tax_rate: f32,
        }
        impl ForGettingTaxRate for MockedGettingTaxRate {
            fn get_tax_rate(&self) -> f32 {
                self.tax_rate
            }
        }
        MockedGettingTaxRate {
            tax_rate
        }
    }
}