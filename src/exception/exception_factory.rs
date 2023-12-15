/***
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/21 13:49
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 **/
use crate::exception::empty_domain_exception::EmptyDomainException;
use crate::exception::ExceptionType;

use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::invalid_variable_exception::InvalidVariableException;
use crate::exception::unsatisfied_constraint_exception::UnsatisfiedConstraintException;
use crate::exception::value_not_found_exception::ValueNotFoundException;

pub struct ExceptionFactory();

impl ExceptionFactory {
    pub fn new(exception_type: ExceptionType, msg: &str) -> Box<dyn ExceptionTrait> {
        match exception_type {
            ExceptionType::EmptyDomainExceptionType => Box::new(EmptyDomainException::new(msg)),
            ExceptionType::UnsatisfiedConstraintException => {
                Box::new(UnsatisfiedConstraintException::new(msg))
            }
            ExceptionType::InvalidVariableExceptionType => {
                Box::new(InvalidVariableException::new(msg))
            }
            ExceptionType::ValueNotFoundExceptionType => Box::new(ValueNotFoundException::new(msg)),
        }
    }
}
