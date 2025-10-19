use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Pricing {
    pub prompt: Decimal,
    pub completion: Decimal,
    pub image: Decimal,
    pub request: Decimal,
    pub web_search: Decimal,
    pub internal_reasoning: Decimal,
    pub input_cache_read: Decimal,
    pub input_cache_write: Decimal,
}

impl Pricing {
    pub fn is_empty(&self) -> bool {
        self.prompt.is_zero()
            && self.completion.is_zero()
            && self.image.is_zero()
            && self.request.is_zero()
            && self.web_search.is_zero()
            && self.internal_reasoning.is_zero()
            && self.input_cache_read.is_zero()
            && self.input_cache_write.is_zero()
    }
}
