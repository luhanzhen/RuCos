/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/8 11:28
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */





#[allow(dead_code)]
pub struct ConstraintFactory();

// impl ConstraintFactory {
//     pub fn new(exception_type: ConstraintFactory, msg: &str) -> Box<dyn ExceptionTrait> {
//         match exception_type {
//             ExceptionType::EmptyDomainExceptionType => Box::new(EmptyDomainException::new(msg)),
//             ExceptionType::UnsatisfiedConstraintException => {
//                 Box::new(UnsatisfiedConstraintException::new(msg))
//             }
//             ExceptionType::InvalidVariableExceptionType => {
//                 Box::new(InvalidVariableException::new(msg))
//             }
//             ExceptionType::ValueNotFoundExceptionType => Box::new(ValueNotFoundException::new(msg)),
//         }
//     }
// }

#[allow(dead_code)]
#[derive(Debug)]
pub enum XConstraintType {
    XConstraintNone,
    XExtension,
    XAllDifferent,
    XAllDifferentExcept,
    XInstantiation,
    XAllEqual,
    XOrdered,
    XRegular,
    XMdd,
    XIntention,
    XGroup,
    XSum,
    XMaximum,
    XMinimum,
    XElement,
    XSlide,
    XCount,
    XNValues,
    XCardinality,
    XChannel,
    XCumulative,
    XNoOverlap,
    XStretch,
    XNoOverlapKDim,
}
