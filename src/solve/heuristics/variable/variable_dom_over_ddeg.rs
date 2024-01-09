use crate::prelude::Var;
use crate::solve::callbacks::delete_decision::DeleteDecision;
use crate::solve::callbacks::new_decision::NewDecision;
use crate::solve::heuristics::variable::heuristic_variable_trait::HeuristicVariableTrait;
use crate::solve::solver::solver::InnerSolver;
use std::fmt::{Debug, Formatter};

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:   2024/1/9 19:44
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
pub(crate) struct VariableDomOverDdeg {}

impl NewDecision for VariableDomOverDdeg {
    fn new_decision_callback(&mut self, var: &Var, solver: &InnerSolver) {
        todo!()
    }

    fn is_implemented(&self) -> bool {
        return true;
    }
}

impl DeleteDecision for VariableDomOverDdeg {
    fn delete_decision_callback(&mut self, var: &Var, value_idx: usize, solver: &InnerSolver) {
        todo!()
    }

    fn full_backtrack(&mut self) {
        todo!()
    }

    fn is_implemented(&self) -> bool {
        return true;
    }
}

impl HeuristicVariableTrait for VariableDomOverDdeg {
    fn select_variable<'a>(&mut self, solver: &'a InnerSolver) -> &'a Var {
        todo!()
    }
}
