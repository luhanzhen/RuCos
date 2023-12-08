/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/8 12:18
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::constraint::propagator::PropagatorTrait;
use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
pub struct CompactTable {}
#[allow(dead_code)]
impl PropagatorTrait for CompactTable {
    fn initialise(&mut self) {
        todo!()
    }

    fn filter_by_variable(&mut self, _dummy: &Rc<RefCell<Variable>>) {
        todo!()
    }

    fn filter_by_arc(&mut self, _dummy: &Rc<RefCell<Variable>>, _value: usize) {
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
