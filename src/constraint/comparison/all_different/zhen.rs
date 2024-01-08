use crate::constraint::propagator::{PropagationPriority, PropagatorTrait};
use crate::exception::exception_trait::ExceptionTrait;
use crate::variable::variable::Var;

/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/22 22:19
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */
#[derive(Debug)]
pub struct GacZhen {
    priority: PropagationPriority,
}

impl PropagatorTrait for GacZhen {
    fn get_priority(&self) -> &PropagationPriority {
        &self.priority
    }
    fn initialise(&mut self) {
        todo!()
    }

    fn filter_by_variable(&mut self, _dummy: &Var) -> Result<usize, &Box<dyn ExceptionTrait>> {
        todo!()
    }

    fn filter_by_arc(
        &mut self,
        _dummy: &Var,
        _value: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>> {
        todo!()
    }

    fn is_coarse_grained(&self) -> bool {
        todo!()
    }

    fn is_fine_grained(&self) -> bool {
        todo!()
    }

    fn restore_to_level(&mut self, _level: usize) {
        todo!()
    }
}
