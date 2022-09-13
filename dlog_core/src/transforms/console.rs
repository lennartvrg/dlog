use time::format_description::FormatItem;
use time::macros::format_description;

use crate::models::Log;
use crate::transforms::Transform;

pub struct ConsoleTransform;

const FORMAT: &[FormatItem] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]");

impl Transform for ConsoleTransform {
    fn apply(&self, log: &mut Log) {
        println!(
            "[{}] [{}]: {}",
            log.timestamp.format(&FORMAT).unwrap(),
            log.priority,
            log.text
        );
    }
}
