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
    factor: f64,
    limit: usize,
    restart_counter: usize,
}
#[allow(dead_code)]
impl LubyRestart {
    pub fn new(solver: &Rc<RefCell<Solver>>, factor: f64) -> Self {
        Self {
            solver: Rc::clone(solver),
            factor,
            limit: 0,
            restart_counter: 0,
        }
    }

    fn luby(&self, x: i32, y: f64) -> f64 {
        let mut s1 = 1;
        let mut s2 = 0;
        loop {
            if s1 < x + 1 {
                break;
            }
            s1 = s1 << 1 + 1;
            s2 += 1;
        }

        y.powi(s2)
    }
}
#[allow(dead_code)]
impl RestartTrait for LubyRestart {
    fn should_restart(&mut self) -> bool {
        let conflicts = self.solver.borrow().get_conflicts();
        if conflicts >= self.limit {
            self.restart_counter += 1;
            self.limit =
                conflicts + (self.luby(2, self.restart_counter as f64) * self.factor) as usize;
            return true;
        }
        false
    }

    fn initialize(&mut self) {
        self.restart_counter = 0;
        self.limit =
            self.solver.borrow().get_conflicts() + (self.luby(2, 0f64) * self.factor) as usize
    }

    fn get_solver(&mut self) -> &mut Rc<RefCell<Solver>> {
        &mut self.solver
    }
}
