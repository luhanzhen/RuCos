use crate::solve::heuristics::value::heuristic_value::HeuristicValueTrait;
use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::rc::Rc;

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:   2023/12/20 15:19
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
#[allow(dead_code)]
pub struct ValueRandom {}

#[allow(dead_code)]
impl ValueRandom {
    pub fn new() -> Self {
        Self {}
    }
}
#[allow(dead_code)]
impl Default for ValueRandom {
    fn default() -> Self {
        ValueRandom::new()
    }
}

#[allow(dead_code)]
impl HeuristicValueTrait for ValueRandom {
    fn select_value(&mut self, var: &Rc<RefCell<Variable>>) -> i32 {
        var.borrow().random_value()
    }

    fn select_idx(&mut self, var: &Rc<RefCell<Variable>>) -> usize {
        var.borrow().random_idx()
    }
}