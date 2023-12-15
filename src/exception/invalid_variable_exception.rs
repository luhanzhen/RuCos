/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/21 13:47
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::ExceptionLevel;
use std::fmt::{Debug, Display, Formatter};

pub struct InvalidVariableException {
    message: String,
    level: ExceptionLevel,
}

impl InvalidVariableException {
    pub fn new(msg: &str) -> Self {
        Self {
            message: String::from(msg),
            level: ExceptionLevel::Major,
        }
    }
}

impl Display for InvalidVariableException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InvalidVariableException: {}, level:{:?}.",
            self.message, self.level
        )
    }
}
impl Debug for InvalidVariableException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl ExceptionTrait for InvalidVariableException {
    #[inline]
    fn message(&self) -> &str {
        &self.message
    }

    #[inline]

    fn exception_level(&self) -> &ExceptionLevel {
        &self.level
    }
}
