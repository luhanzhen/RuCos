/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/16 14:18
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::ExceptionLevel;
use std::fmt::{Display, Formatter};

pub struct ValueNotFoundException {
    message: String,
    level: ExceptionLevel,
}

impl ValueNotFoundException {
    pub fn new(msg: &str) -> Self {
        Self {
            message: String::from(msg),
            level: ExceptionLevel::Major,
        }
    }
}

impl Display for ValueNotFoundException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ValueNotFoundException: {}, level:{:?}.",
            self.message, self.level
        )
    }
}
impl ExceptionTrait for ValueNotFoundException {
    #[inline]
    fn message(&self) -> &str {
        &self.message
    }
    #[inline]
    fn exception_level(&self) -> &ExceptionLevel {
        &self.level
    }
}
