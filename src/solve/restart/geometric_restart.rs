use crate::solve::restart::restart_trait::RestartTrait;
use crate::solve::solver::solver::Solver;
use rand::random;

/* * *
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
 * * */

#[allow(dead_code)]
#[derive(Debug)]
pub struct GeometricRestart {
    factor: u64,
    limit: u64,
    restart_counter: u64,
}

#[allow(dead_code)]
impl GeometricRestart {
    pub fn new_with_solver_and_factor(factor: u64) -> Self {
        Self {
            factor,
            // limit: solver.borrow().get_conflicts() as u64,
            limit: 0,
            restart_counter: 100,
        }
    }

    pub fn new_with_solver_and_random_factor() -> Self {
        GeometricRestart::new_with_solver_and_factor(random::<u64>() % 100 + 1)
    }
}
#[allow(dead_code)]
impl RestartTrait for GeometricRestart {
    fn should_restart(&mut self, solver: &mut Solver) -> bool {
        let conflicts = solver.get_conflicts() as u64;

        if conflicts >= self.limit {
            self.restart_counter *= self.factor;
            self.limit += conflicts;
            true
        } else {
            false
        }
    }

    fn initialize(&mut self, solver: &mut Solver) {
        self.restart_counter = 100;
        self.limit = solver.get_conflicts() as u64
    }
}
