use std::time::SystemTime;

pub struct TimeManager;

impl TimeManager {
    pub(crate) fn should_stop(start_time: SystemTime, move_time: Option<u64>) -> bool {
        if let Some(move_time) = move_time {
            let elapsed_time = SystemTime::now()
                .duration_since(start_time)
                .unwrap()
                .as_millis() as u64;

            if elapsed_time >= move_time / 25 {
                return true;
            }
        }

        false
    }
}
