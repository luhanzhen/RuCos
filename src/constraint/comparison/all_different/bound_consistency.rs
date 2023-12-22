use crate::constraint::propagator::PropagatorTrait;
use crate::exception::exception_trait::ExceptionTrait;
use crate::problem::problem::Var;

/**
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
 */

pub struct BoundConsistency {

}

impl PropagatorTrait for BoundConsistency {
    fn initialise(&mut self) {
        todo!()
    }

    fn filter_by_variable(&mut self, dummy: &Var) -> Result<usize, &Box<dyn ExceptionTrait>> {
        todo!()
    }

    fn filter_by_arc(
        &mut self,
        dummy: &Var,
        value: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>> {
        todo!()
    }

    fn is_coarse_grained(&self) -> bool {
        todo!()
    }

    fn is_fine_grained(&self) -> bool {
        todo!()
    }

    fn restore_to_level(&mut self) {
        todo!()
    }
}
