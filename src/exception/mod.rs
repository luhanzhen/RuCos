/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/16 13:21
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
pub mod empty_domain_exception;
pub mod exception_factory;
pub mod exception_trait;
pub mod invalid_variable_exception;
pub mod unsatisfied_constraint_exception;
pub mod value_not_found_exception;

#[derive(Debug)]
pub enum ExceptionType {
    EmptyDomainExceptionType,
    ValueNotFoundExceptionType,
    InvalidVariableExceptionType,
    UnsatisfiedConstraintException,
}

#[derive(Debug)]
pub enum ExceptionLevel {
    Ignorable,
    Minor,
    Major, // you have to handle this error
    Fatal, // you have to handle this error
}
