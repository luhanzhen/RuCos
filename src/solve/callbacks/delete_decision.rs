use crate::solve::solver::solver::Solver;
use crate::variable::variable::Var;

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/23 23:55
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */

pub trait DeleteDecision {
    fn delete_decision_callback(&mut self, var: &Var, value_idx: usize, solver: &Solver);
}
