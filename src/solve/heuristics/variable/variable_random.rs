use crate::prelude::Var;
use crate::solve::callbacks::delete_decision::DeleteDecision;
use crate::solve::callbacks::new_decision::NewDecision;
use crate::solve::heuristics::variable::heuristic_variable_trait::HeuristicVariableTrait;
use crate::solve::solver::solver::InnerSolver;
use rand::random;
use std::fmt::{Debug, Formatter};

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:   2024/1/9 20:43
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct VariableRandom {}

impl NewDecision for VariableRandom {
    fn new_decision_callback(&mut self, var: &Var, solver: &InnerSolver) {
        todo!()
    }
}

impl DeleteDecision for VariableRandom {
    fn delete_decision_callback(&mut self, var: &Var, value_idx: usize, solver: &InnerSolver) {
        todo!()
    }

    fn full_backtrack(&mut self) {
        todo!()
    }
}

impl HeuristicVariableTrait for VariableRandom {
    fn select_variable<'a>(&mut self, solver: &'a InnerSolver) -> &'a Var {
        let len = solver.borrow().future_vars.len();
        let random = random::<usize>() % len;
        let borrow = solver.borrow();
        todo!()
    }
}
