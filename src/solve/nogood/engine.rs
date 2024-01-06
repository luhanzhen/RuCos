/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/24 00:09
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */

use crate::prelude::Var;
use crate::solve::callbacks::delete_decision::DeleteDecision;
use crate::solve::callbacks::new_decision::NewDecision;

use std::collections::HashMap;
use std::fmt::Debug;

use crate::solve::solver::solver::InnerSolver;

#[allow(dead_code)]
#[derive(Debug)]
pub struct NoGoodEngine {
    solver: InnerSolver,
    current_branch: Vec<u128>,
    nogood_equals_to_one: Vec<u128>,
    watcher_position: HashMap<u128, u32>,
    watcher: Vec<Vec<u128>>,
    capacity: usize,
}

#[allow(dead_code)]
impl NoGoodEngine {
    pub fn new(solver: &InnerSolver) -> Self {
        Self {
            solver: solver.clone(),
            current_branch: vec![],
            nogood_equals_to_one: vec![],
            watcher_position: Default::default(),
            watcher: vec![],
            capacity: 0,
        }
    }
}

impl NewDecision for NoGoodEngine {
    fn new_decision_callback(&mut self, var: &Var, solver: &InnerSolver) {
        todo!()
    }
}

impl DeleteDecision for NoGoodEngine {
    fn delete_decision_callback(&mut self, var: &Var, value_idx: usize, solver: &InnerSolver) {
        todo!()
    }
}
