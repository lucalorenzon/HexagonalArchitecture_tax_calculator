use crate::domain::driven_port::for_getting_tax_rate::ForGettingTaxRate;

pub struct TaxRateRepository {
    tax_rate: f32,
}

impl TaxRateRepository {
    pub(crate) fn new(tax_rate: f32) -> Self {
        TaxRateRepository {
            tax_rate
        }
    }
}

impl ForGettingTaxRate for TaxRateRepository {
    fn get_tax_rate(&self) -> f32 {
        self.tax_rate
    }
}