use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/16 14:37
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */

pub trait HeuristicVariableTrait {
    fn select_variable(&mut self, future_vars: &HashSet<Rc<RefCell<Variable>>>) -> i32;
}
