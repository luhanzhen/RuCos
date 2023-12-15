/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/7 16:16
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::solve::solver::solver::Solver;
use std::cell::RefCell;
use std::rc::Rc;

pub trait RestartTrait {
    fn should_restart(&self) -> bool;

    fn initialize(&mut self);

    fn get_solver(&mut self) -> &mut Rc<RefCell<Solver>>;
}
