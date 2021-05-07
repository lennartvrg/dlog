mod console;
mod email;

use crate::models::Log;
use crate::transforms::console::ConsoleTransform;

pub trait Transform {
    fn apply(&self, log: &mut Log);
}

pub struct Transforms {
    transforms: Vec<Box<dyn Transform>>
}

impl Transforms {
    pub fn new() -> Self {
        Self {
            transforms: vec![
                Box::new(ConsoleTransform)
            ]
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