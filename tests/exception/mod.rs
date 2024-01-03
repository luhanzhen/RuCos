/* * *
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
 * * */
use rucos::exception::exception_factory::ExceptionFactory;

use rucos::exception::{ExceptionLevel, ExceptionType};

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
