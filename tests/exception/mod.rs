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

use constraint_solver::exception::exception_factory::ExceptionFactory;

use constraint_solver::exception::{ExceptionLevel, ExceptionType};

#[test]
pub fn test_factory() {
    let exception = ExceptionFactory::new(ExceptionType::EmptyDomainExceptionType, "empty domain");

    assert_eq!(exception.message(), "empty domain");
    match exception.exception_level() {
        ExceptionLevel::Fatal => {}
        _ => {
            panic!("match exception level error!!")
        }
    }

    let exception = ExceptionFactory::new(ExceptionType::InvalidVariableExceptionType, "");
    assert_eq!(exception.message(), "");
    match exception.exception_level() {
        ExceptionLevel::Major => {}
        _ => {
            panic!("match exception level error!!")
        }
    }
}
