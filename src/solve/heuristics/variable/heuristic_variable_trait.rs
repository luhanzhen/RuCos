use crate::prelude::Var;
use crate::solve::callbacks::delete_decision::DeleteDecision;
use crate::solve::callbacks::new_decision::NewDecision;
use crate::solve::solver::solver::InnerSolver;
use std::fmt::Debug;

/* * *
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
 * * */

pub trait HeuristicVariableTrait: Debug + NewDecision + DeleteDecision {
    fn select_variable<'a>(&mut self, solver: &'a InnerSolver) -> &'a Var;
}
