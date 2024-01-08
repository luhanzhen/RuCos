/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/16 14:14
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */
use crate::constraint::propagator::PropagationPriority::Unary;
use crate::exception::exception_trait::ExceptionTrait;
use crate::variable::variable::Var;
use std::fmt::Debug;

pub(crate) trait PropagatorTrait: Debug {
    fn initialise(&mut self);
    fn filter_by_variable(&mut self, dummy: &Var) -> Result<usize, &Box<dyn ExceptionTrait>>;
    fn filter_by_arc(
        &mut self,
        dummy: &Var,
        value: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>>;
    fn is_coarse_grained(&self) -> bool;
    fn is_fine_grained(&self) -> bool;
    fn restore_to_level(&mut self, level: usize);

    fn get_priority(&self) -> &PropagationPriority;
}
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum PropagationGrained {
    Coarse,
    Fine,
    Both,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum PropagationPriority {
    Unary,
    Binary,
    Ternary,
    Linear,
    Quadratic,
    Cubic,
    VerySlow,
}

#[allow(dead_code)]
impl PropagationPriority {
    pub fn new(prio: usize) -> Self {
        match prio {
            1 => Unary,
            2 => Self::Binary,
            3 => Self::Ternary,
            4 => Self::Linear,
            5 => Self::Quadratic,
            6 => Self::Cubic,
            _ => Self::VerySlow,
        }
    }
}

impl PartialEq<usize> for PropagationPriority {
    fn eq(&self, other: &usize) -> bool {
        let left: usize = match self {
            Unary => 1,
            PropagationPriority::Binary => 2,
            PropagationPriority::Ternary => 3,
            PropagationPriority::Linear => 4,
            PropagationPriority::Quadratic => 5,
            PropagationPriority::Cubic => 6,
            PropagationPriority::VerySlow => 7,
        };
        left == *other
    }
}
