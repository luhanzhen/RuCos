/**
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/21 13:49
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 **/
use crate::exception::empty_domain_exception::EmptyDomainException;
use crate::exception::ExceptionType;

use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::invalid_variable_exception::InvalidVariableException;

pub struct ExceptionFactory();

impl ExceptionFactory {
    pub fn new(exception_type: ExceptionType, msg: &str) -> Box<dyn ExceptionTrait> {
        match exception_type {
            ExceptionType::EmptyDomainExceptionType => Box::new(EmptyDomainException::new(msg)),
            ExceptionType::InvalidVariableExceptionType => {
                Box::new(InvalidVariableException::new(msg))
            }
        }
    }
}
