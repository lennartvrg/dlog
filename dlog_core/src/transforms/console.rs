use crate::transforms::Transform;
use crate::models::Log;

pub struct ConsoleTransform;

impl Transform for ConsoleTransform {
    fn apply(&self, log: &mut Log) {
        println!("[{}] [{}]: {}", log.timestamp, log.priority, log.message);
    }
}