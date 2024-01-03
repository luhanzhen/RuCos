/* * *
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
 * * */
use crate::solve::restart::restart_trait::RestartTrait;
use crate::solve::solver::solver::Solver;
use rand::random;

use crate::solve::seal::Seal;
#[derive(Debug)]
pub struct LubyRestart {
    solver: Seal<Solver>,
    factor: u64,
    limit: u64,
    restart_counter: u64,
}

fn luby(mut x: u64, y: u64) -> u64 {
    let mut s1 = 1;
    let mut seq = 0;
    loop {
        if s1 > x {
            break;
        }
        s1 <<= 1 + 1;
        seq += 1;
    }
    while s1 - 1 != x {
        s1 = (s1 - 1) >> 1;
        seq -= 1;
        x %= s1
    }

    y.pow(seq)
}

#[allow(dead_code)]
impl LubyRestart {
    pub fn new_with_solver_and_factor(solver: &Seal<Solver>, factor: u64) -> Self {
        Self {
            solver: solver.clone(),
            factor,
            limit: luby(2, 0),
            restart_counter: 0,
        }
    }
    pub fn new_with_solver_and_random_factor(solver: &Seal<Solver>) -> Self {
        LubyRestart::new_with_solver_and_factor(solver, random::<u64>() % 100 + 1)
    }
}
#[allow(dead_code)]
impl RestartTrait for LubyRestart {
    fn should_restart(&mut self) -> bool {
        let conflicts = self.solver.borrow().get_conflicts() as u64;
        if conflicts >= self.limit {
            self.restart_counter += 1;
            self.limit = conflicts + (luby(2, self.restart_counter) * self.factor);
            true
        } else {
            false
        }
    }

    fn initialize(&mut self) {
        self.restart_counter = 0;
        self.limit =
            (self.solver.borrow().get_conflicts() + (luby(2, 0) * self.factor) as usize) as u64
    }
}
