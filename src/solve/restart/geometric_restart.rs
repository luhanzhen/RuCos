use crate::solve::restart::restart_trait::RestartTrait;
use crate::solve::solver::solver::Solver;
use rand::random;

use crate::solve::seal::Seal;

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
    solver: Seal<Solver>,
    factor: u64,
    limit: u64,
    restart_counter: u64,
}

#[allow(dead_code)]
impl GeometricRestart {
    pub fn new_with_solver_and_factor(solver: &Seal<Solver>, factor: u64) -> Self {
        Self {
            solver: solver.clone(),
            factor,
            limit: solver.borrow().get_conflicts() as u64,
            restart_counter: 100,
        }
    }

    pub fn new_with_solver_and_random_factor(solver: &Seal<Solver>) -> Self {
        GeometricRestart::new_with_solver_and_factor(solver, random::<u64>() % 100 + 1)
    }
}
#[allow(dead_code)]
impl RestartTrait for GeometricRestart {
    fn should_restart(&mut self) -> bool {
        let conflicts = self.solver.borrow().get_conflicts() as u64;
        if conflicts >= self.limit {
            self.restart_counter *= self.factor;
            self.limit += conflicts;
            true
        } else {
            false
        }
    }

    fn initialize(&mut self) {
        self.restart_counter = 100;
        self.limit = self.solver.borrow().get_conflicts() as u64
    }
}
