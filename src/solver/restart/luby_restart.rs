/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/7 16:19
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::solver::restart::restart_trait::RestartTrait;
use crate::solver::solver::solver::Solver;
use std::cell::RefCell;
use std::rc::Rc;

pub struct LubyRestart {
    solver: Rc<RefCell<Solver>>,
}
#[allow(dead_code)]
impl LubyRestart {
    pub fn new(solver: &Rc<RefCell<Solver>>) -> Self {
        Self {
            solver: solver.clone(),
        }
    }
}
#[allow(dead_code)]
impl RestartTrait for LubyRestart {
    fn should_restart(&self) -> bool {
        todo!()
    }

    fn initialize(&mut self) {
        todo!()
    }

    fn get_solver(&mut self) -> &mut Rc<RefCell<Solver>> {
        &mut self.solver
    }
}
