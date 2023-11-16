/*
 * <p>@project_name: constraint_solver
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

use crate::exception::exception_trait::{ExceptionLevel, ExceptionTrait};

struct EmptyDomainException {
    message: String,
    level: ExceptionLevel,
}

impl ExceptionTrait for EmptyDomainException {
    fn message(&self) -> &str {
        &self.message
    }

    fn new(msg: &str) -> Self {
        Self {
            message: String::from(msg),
            level: ExceptionLevel::Fatal,
        }
    }

    fn exception_level(&self) -> &ExceptionLevel {
        &self.level
    }
}
