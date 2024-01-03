use crate::solve::heuristics::value::heuristic_value::HeuristicValueTrait;
use crate::variable::variable::Var;

/* * *
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
 * * */

#[allow(dead_code)]
#[derive(Debug)]
pub struct ValueFirst {}

#[allow(dead_code)]
impl ValueFirst {
    pub fn new() -> Self {
        Self {}
    }
}
#[allow(dead_code)]
impl Default for ValueFirst {
    fn default() -> Self {
        ValueFirst::new()
    }
}
#[allow(dead_code)]
impl HeuristicValueTrait for ValueFirst {
    fn select_value(&mut self, var: &Var) -> i32 {
        var.borrow().minimum_value()
    }

    fn select_idx(&mut self, var: &Var) -> usize {
        var.borrow().minimum_idx()
    }
}
