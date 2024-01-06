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
    pub(crate) construction_timer: Duration,
    pub(crate) init_time: Duration,
}

impl Clone for TimeComponent {
    fn clone(&self) -> Self {
        Self {
            timer: self.timer.clone(),
            init_time: self.init_time,
            construction_timer: Duration::default(),
        }
    }
}
#[allow(dead_code)]
impl TimeComponent {
    pub(crate) fn new(du: Duration) -> Self {
        Self {
            timer: Default::default(),
            construction_timer: Duration::default(),
            init_time: du,
        }
    }
    pub(crate) fn reset(&mut self) {
        self.timer.reset();
    }

    pub(crate) fn get_problem_set_time(&self) -> Duration {
        self.init_time
    }

    pub(crate) fn get_solver_construction_time(&self) -> Duration {
        self.construction_timer
    }
    pub(crate) fn set_solver_construction_time(&mut self) {
        self.construction_timer = self.timer.get();
        self.timer.reset()
    }
    pub(crate) fn get_time_interval(&self) -> Duration {
        self.timer.get()
    }

    pub(crate) fn get_timer(&self) -> &TimeInterval {
        &self.timer
    }
}
