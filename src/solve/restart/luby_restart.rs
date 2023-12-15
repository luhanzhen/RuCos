/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/7 16:19
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::solve::restart::restart_trait::RestartTrait;
use crate::solve::solver::solver::Solver;
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
