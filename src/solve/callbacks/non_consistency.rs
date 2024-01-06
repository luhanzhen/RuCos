use crate::constraint::constraint::Constraint;
use crate::solve::solver::solver::InnerSolver;

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

pub trait NonConsistency: Debug {
    fn non_consistency_callback(&mut self, cons: &Constraint, level: usize, solver: &InnerSolver);
}
