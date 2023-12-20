use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::rc::Rc;

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/16 14:36
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */

pub trait HeuristicValueTrait {
    fn select_value(&mut self, var: &Rc<RefCell<Variable>>) -> i32;
    fn select_idx(&mut self, var: &Rc<RefCell<Variable>>) -> usize;
}
