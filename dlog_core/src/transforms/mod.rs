mod console;
mod credit_card;
mod email;

use crate::models::Log;
use crate::transforms::console::ConsoleTransform;
use crate::transforms::credit_card::CreditCardTransform;
use crate::transforms::email::EmailTransform;

pub trait Transform: Send {
    fn apply(&self, log: &mut Log);
}

pub struct Transforms {
    transforms: Vec<Box<dyn Transform>>,
}

impl Transforms {
    pub fn new() -> Self {
        Self {
            transforms: vec![
                Box::new(CreditCardTransform),
                Box::new(EmailTransform),
                Box::new(ConsoleTransform),
            ],
        }
    }
}

impl Transform for Transforms {
    fn apply(&self, log: &mut Log) {
        for transform in &self.transforms {
            transform.apply(log)
        }
    }
}
