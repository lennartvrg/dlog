use regex::Regex;
use static_init::dynamic;
use std::fmt::Write;

use crate::models::Log;
use crate::transforms::Transform;

#[dynamic]
static NUMERIC: Regex = Regex::new(r#"^\d+$"#).unwrap();

#[dynamic]
static CREDIT_CARD: Regex = Regex::new(r#"^(?:4[0-9]{12}(?:[0-9]{3})?|[25][1-7][0-9]{14}|6(?:011|5[0-9][0-9])[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|(?:2131|1800|35\d{3})\d{11})$"#).unwrap();

pub struct CreditCardTransform;

impl Transform for CreditCardTransform {
    fn apply(&self, log: &mut Log) {
        let mut counter = 0;
        let mut message = Vec::<String>::new();
        for part in log.text.split(&[' ', '-'][..]) {
            if CREDIT_CARD.is_match(part) {
                message.push("•".repeat(16));
            } else if NUMERIC.is_match(part) {
                message.push(part.to_owned());
                counter += 1;
                if counter == 4 {
                    let mut last_four = get_last_four(&mut message);
                    if CREDIT_CARD.is_match(&last_four.join("")) {
                        last_four = vec!["•".repeat(16)];
                        counter = 0;
                    }
                    message.append(&mut last_four);
                }
            } else if part.trim().is_empty() {
                write!(message.last_mut().unwrap(), "{} ", part).unwrap();
            } else {
                message.push(part.to_owned());
                counter = 0;
            }
        }
        log.text = message.join(" ");
    }
}

fn get_last_four(message: &mut Vec<String>) -> Vec<String> {
    message
        .split_off(message.len() - 4)
        .iter()
        .map(|val| val.trim().to_owned())
        .collect::<Vec<String>>()
}
