use crate::constraint::constraint::ConstraintTrait;
use crate::solve::solver::solver::Solver;
use std::cell::RefCell;
use std::rc::Rc;

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

pub trait NonConsistency {
    fn non_consistency_callback(
        &mut self,
        cons: &Rc<RefCell<dyn ConstraintTrait>>,
        level: usize,
        solver: &Solver,
    );
}
