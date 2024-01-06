use crate::solve::solver::solver::InnerSolver;
use crate::variable::variable::Var;
use std::fmt::Debug;

/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/23 23:56
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */

pub trait NewDecision: Debug {
    fn new_decision_callback(&mut self, var: &Var, solver: &InnerSolver);
}
