/**
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/10 20:08
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
use std::time::{Duration, Instant};

pub struct TimeInterval {
    start: Instant,
}

#[allow(dead_code)]
impl TimeInterval {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    pub fn get(&self) -> Duration {
        self.start.elapsed()
    }
}

impl Default for TimeInterval {
    fn default() -> Self {
        Self::new()
    }
}
