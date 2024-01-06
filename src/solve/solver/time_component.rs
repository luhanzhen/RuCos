/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/1/5 23:27
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */

use crate::utils::time_interval::TimeInterval;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct TimeComponent {
    pub(crate) timer: TimeInterval,
    pub(crate) init_time: Option<Duration>,
}

impl TimeComponent {
    pub(crate) fn new() -> Self {
        Self {
            timer: Default::default(),
            init_time: None,
        }
    }
    pub(crate) fn init(&mut self, timer: Duration) {
        self.init_time = Some(timer);
        self.timer.reset();
    }

    pub(crate) fn get_problem_set_time(&self) -> Duration {
        self.init_time.unwrap()
    }
    pub(crate) fn get_time_interval(&self) -> Duration {
        self.timer.get()
    }

    pub(crate) fn get_timer(&self) -> &TimeInterval {
        &self.timer
    }
}
