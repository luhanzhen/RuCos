/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/16 13:12
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::exception::ExceptionLevel;
use std::fmt::{Debug, Display};

pub trait ExceptionTrait: Display + Debug {
    fn message(&self) -> &str;

    fn exception_level(&self) -> &ExceptionLevel;

    #[inline]
    fn is_fatal(&self) -> bool {
        match self.exception_level() {
            ExceptionLevel::Major => true,
            ExceptionLevel::Fatal => true,
            _ => false,
        }
    }
}
