use crate::solve::heuristics::value::heuristic_value::HeuristicValueTrait;
use crate::variable::variable::Var;

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
#[derive(Debug)]
pub struct ValueSticking {
    last_value: Vec<usize>,
    heuristic: Box<dyn HeuristicValueTrait>,
}
#[allow(dead_code)]
impl ValueSticking {
    pub fn new(n_variable: usize, heuristic: Box<dyn HeuristicValueTrait>) -> Self {
        let mut last_value = vec![];
        last_value.resize(n_variable, usize::MAX);
        Self {
            last_value,
            heuristic,
        }
    }
}

#[allow(dead_code)]
impl HeuristicValueTrait for ValueSticking {
    fn select_value(&mut self, var: &Var) -> i32 {
        return if let Some(v) = var.borrow().idx_to_value(self.select_idx(var)) {
            v
        } else {
            i32::MAX
        };
    }

    fn select_idx(&mut self, var: &Var) -> usize {
        return if var.borrow().domain_size() == 1 {
            let n = var.borrow()[0];
            self.last_value[var.borrow().get_id()] = n;
            n
        } else {
            let lv = self.last_value[var.borrow().get_id()];
            if lv != usize::MAX && var.borrow().contains_idx(lv) {
                return lv;
            }
            self.heuristic.select_idx(var)
        };
    }
}
