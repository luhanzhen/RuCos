/*
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/21 13:47
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

impl ExceptionTrait for InvalidVariableException {
    fn message(&self) -> &str {
        &self.message
    }

    fn exception_level(&self) -> &ExceptionLevel {
        &self.level
    }
}
