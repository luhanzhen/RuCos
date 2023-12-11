/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/7 16:16
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::solve::solver::solver::Solver;
use std::cell::RefCell;
use std::rc::Rc;

pub trait RestartTrait {
    fn should_restart(&self) -> bool;

    fn initialize(&mut self);

    fn get_solver(&mut self) -> &mut Rc<RefCell<Solver>>;
}
