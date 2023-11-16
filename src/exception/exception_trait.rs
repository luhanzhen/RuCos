/*
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/16 13:12
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

pub enum ExceptionLevel {
    Ignorable,
    Minor,
    Major, // you have to handle this error
    Fatal, // you have to handle this error
}

pub trait ExceptionTrait {
    fn message(&self) -> &str;

    fn new() -> Self;

    fn exception_level(&self) -> &ExceptionLevel;

    fn is_fatal(&self) -> bool {
        match self.exception_level() {
            ExceptionLevel::Major => true,
            ExceptionLevel::Fatal => true,
            _ => false,
        }
    }
}
