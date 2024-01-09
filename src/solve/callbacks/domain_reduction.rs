use crate::solve::solver::solver::InnerSolver;
use crate::variable::variable::Var;
use std::fmt::Debug;

/* * *
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
 * * */

pub trait DomainReduction: Debug {
    fn domain_reduction_callback(&mut self, var: &Var, value_idx: usize, solver: &InnerSolver);
    fn domain_assignment_callback(&mut self, var: &Var, value_idx: usize, solver: &InnerSolver);

    fn is_implemented(&self) -> bool {
        return false;
    }
}
