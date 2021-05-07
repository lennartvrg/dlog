use crate::models::Log;
use crate::transforms::Transform;

pub struct ConsoleTransform;

impl Transform for ConsoleTransform {
    fn apply(&self, log: &mut Log) {
        println!("[{}] [{}]: {}", log.timestamp, log.priority, log.message);
    }
}
