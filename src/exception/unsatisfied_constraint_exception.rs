/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/26 23:53
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

pub struct UnsatisfiedConstraintException {
    message: String,
    level: ExceptionLevel,
}

impl UnsatisfiedConstraintException {
    pub fn new(msg: &str) -> Self {
        Self {
            message: String::from(msg),
            level: ExceptionLevel::Fatal,
        }
    }
}

impl Display for UnsatisfiedConstraintException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UnsatisfiedConstraintException: {}, level:{:?}.",
            self.message, self.level
        )
    }
}

impl Debug for UnsatisfiedConstraintException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl ExceptionTrait for UnsatisfiedConstraintException {
    #[inline]
    fn message(&self) -> &str {
        &self.message
    }
    #[inline]
    fn exception_level(&self) -> &ExceptionLevel {
        &self.level
    }
}
