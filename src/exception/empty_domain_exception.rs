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

pub struct EmptyDomainException {
    message: String,
    level: ExceptionLevel,
}
impl EmptyDomainException {
    pub fn new(msg: &str) -> Self {
        Self {
            message: String::from(msg),
            level: ExceptionLevel::Fatal,
        }
    }
}

impl ExceptionTrait for EmptyDomainException {
    #[inline]
    fn message(&self) -> &str {
        &self.message
    }
    #[inline]
    fn exception_level(&self) -> &ExceptionLevel {
        &self.level
    }
}
