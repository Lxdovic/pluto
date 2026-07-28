use crate::search::search_options::BuiltSearchOptions;
use std::time::SystemTime;

pub struct TimeManager;

impl TimeManager {
    pub(crate) fn should_stop(start_time: SystemTime, opt: &BuiltSearchOptions) -> bool {
        if let Some(move_time) = opt.time {
            let elapsed_time = SystemTime::now()
                .duration_since(start_time)
                .unwrap()
                .as_millis() as u64;

            if elapsed_time >= move_time {
                return true;
            }
        }

        false
    }
}
