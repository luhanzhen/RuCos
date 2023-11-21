/*
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/16 13:21
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

pub mod empty_domain_exception;
pub mod exception_factory;
pub mod exception_trait;
pub mod invalid_variable_exception;

pub enum ExceptionType {
    EmptyDomainExceptionType,
    InvalidVariableExceptionType,
}
pub enum ExceptionLevel {
    Ignorable,
    Minor,
    Major, // you have to handle this error
    Fatal, // you have to handle this error
}
