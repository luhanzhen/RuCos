use crate::solve::heuristics::value::heuristic_value::HeuristicValueTrait;
use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::rc::Rc;

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:   2023/12/21 14:51
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */

pub struct ValueSticking {
    last_value: Vec<usize>,
}

impl ValueSticking {
    pub fn new() -> Self {
        Self { last_value: vec![] }
    }
}

#[allow(dead_code)]
impl HeuristicValueTrait for ValueSticking {
    fn select_value(&mut self, var: &Rc<RefCell<Variable>>) -> i32 {
        todo!()
    }

    fn select_idx(&mut self, var: &Rc<RefCell<Variable>>) -> usize {
        todo!()
    }
}
