use crate::solve::restart::restart_trait::RestartTrait;
use crate::solve::solver::solver::Solver;
use std::cell::RefCell;
use std::rc::Rc;

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:   2023/12/18 20:58
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */

#[allow(dead_code)]
pub struct GeometricRestart {
    solver: Rc<RefCell<Solver>>,
    factor: u64,
    limit: u64,
    restart_counter: u64,
}

#[allow(dead_code)]
impl GeometricRestart {
    pub fn new(solver: &Rc<RefCell<Solver>>, factor: u64) -> Self {
        Self {
            solver: Rc::clone(solver),
            factor,
            limit: 0,
            restart_counter: 0,
        }
    }
}
#[allow(dead_code)]
impl RestartTrait for GeometricRestart {
    fn should_restart(&mut self) -> bool {
        todo!()
    }

    fn initialize(&mut self) {
        todo!()
    }
}
