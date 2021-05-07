use regex::Regex;
use static_init::dynamic;

use crate::models::Log;
use crate::transforms::Transform;

#[dynamic]
static CREDIT_CARD: Regex = Regex::new(r#"^(?:4[0-9]{12}(?:[0-9]{3})?|(?:5[1-5][0-9]{2}|222[1-9]|22[3-9][0-9]|2[3-6][0-9]{2}|27[01][0-9]|2720)[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|6(?:011|5[0-9]{2})[0-9]{12}|(?:2131|1800|35\d{3})\d{11})$"#).unwrap();

pub struct CreditCardTransform;

impl Transform for CreditCardTransform {
    fn apply(&self, log: &mut Log) {
        log.message = log
            .message
            .split(' ')
            .map(|val| match CREDIT_CARD.is_match(val) {
                true => std::iter::repeat("•".repeat(4)).collect::<Vec<String>>().join("-"),
                false => String::from(val),
            })
            .collect::<Vec<String>>()
            .join(" ");
    }
}
