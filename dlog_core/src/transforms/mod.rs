mod console;
mod credit_card;
mod email;

use crate::models::Log;
use crate::transforms::console::ConsoleTransform;
use crate::transforms::credit_card::CreditCardTransform;
use crate::transforms::email::EmailTransform;

pub trait Transform: Send + Sync {
    fn apply(&self, log: &mut Log);
}

pub struct Transforms {
    transforms: Vec<Box<dyn Transform>>,
}

impl Transforms {
    pub fn new() -> Self {
        Self {
            transforms: vec![Box::new(ConsoleTransform)],
        }
    }

    pub fn add_credit_card_sanitizer(&mut self, add: bool) {
        if add {
            self.transforms.insert(0, Box::new(CreditCardTransform));
        }
    }

    pub fn add_email_sanitizer(&mut self, add: bool) {
        if add {
            self.transforms.insert(0, Box::new(EmailTransform));
        }
    }
}

impl Default for Transforms {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform for Transforms {
    fn apply(&self, log: &mut Log) {
        for transform in &self.transforms {
            transform.apply(log)
        }
    }
}
