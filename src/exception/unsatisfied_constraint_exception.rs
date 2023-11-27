/*
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/26 23:53
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
