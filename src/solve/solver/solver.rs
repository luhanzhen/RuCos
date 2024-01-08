/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/2 20:08
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */

use crate::constraint::propagator::PropagatorTrait;
use crate::problem::problem::Problem;
use crate::solve::seal::Seal;
use crate::solve::solution::Solution;
use crate::solve::solver::callback_set::CallbackSet;
use crate::solve::solver::core_component::CoreComponent;
use crate::solve::solver::heuristic_component::HeuristicComponent;
use crate::solve::solver::status_component::*;
use crate::solve::solver::time_component::TimeComponent;
use crate::variable::variable::Var;
use rand::prelude::*;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub type InnerSolver = Seal<CoreComponent>;
#[allow(dead_code)]
#[derive(Debug)]
pub struct Solver {
    solutions: Solution,
    status_component: StatusComponent,
    core_component: Seal<CoreComponent>,
    time_component: TimeComponent,
    heuristic_component: HeuristicComponent,
    callback_set: CallbackSet,
    queue: VecDeque<Rc<dyn PropagatorTrait>>,
}

#[allow(dead_code)]
impl Solver {
    fn delay_construct(&mut self) {
        self.core_component
            .borrow()
            .do_something_constraint(|c| c.borrow_mut().delay_construct(&self.core_component));
        self.core_component
            .borrow()
            .add_constraint_to_variable_scoped();
    }

    fn choose_strategy(&mut self) {
        self.heuristic_component.choose_strategy()
    }

    fn decide_the_variable_with_idx(&self, var: &Var, idx: usize) {
        let _ = var
            .borrow_mut()
            .assign_idx(idx, self.core_component.borrow().level);
        var.borrow_mut()
            .record_limit(self.core_component.borrow().level)
    }

    fn propagate(&mut self) {
        while !self.queue.is_empty() {
            if let Some(mut element) = self.queue.pop_back() {
                let p = element.get_priority();
                element.restore_to_level(0)
            }
        }
    }

    fn first_propagate(&mut self) {}

    fn backtrack_to_level(&mut self, _level: usize) {}
    fn backtrack(&mut self) {}
}

#[allow(dead_code)]
impl Solver {
    pub(crate) fn get_conflicts(&self) -> usize {
        self.core_component.borrow().conflicts
    }

    // pub(crate) fn get_future_vars(&self) -> &HashSet<Var> {
    //     &self.core_component.borrow().future_vars
    // }
    //
    // pub(crate) fn get_past_vars(&self) -> &HashSet<Var> {
    //     &self.core_component.borrow().past_vars
    // }

    pub(crate) fn get_level(&self) -> usize {
        self.core_component.borrow().level
    }
}
impl Clone for Solver {
    fn clone(&self) -> Self {
        println!("Cloning Solver");
        Self {
            solutions: self.solutions.clone(),
            time_component: self.time_component.clone(),
            core_component: self.core_component.clone(),
            heuristic_component: self.heuristic_component.clone(),
            callback_set: CallbackSet::new(),
            status_component: self.status_component.clone(),
            queue: Default::default(),
        }
    }
}

#[allow(dead_code)]
impl Solver {
    pub fn new(problem: &Problem) -> Solver {
        let tmp_cons = problem.get_constraints().clone();
        let tmp_var = problem.get_all_variables().clone();
        let solutions = Solution::new(&tmp_var);
        let core = CoreComponent::new(tmp_var, tmp_cons);
        Self {
            solutions,
            core_component: Seal::new(core),
            time_component: TimeComponent::new(problem.time()),
            heuristic_component: HeuristicComponent::new(),
            callback_set: CallbackSet::new(),
            status_component: StatusComponent::new(),
            queue: Default::default(),
        }
    }

    pub fn solve(&mut self) {
        self.time_component.reset();
        self.choose_strategy();
        self.core_component.borrow_mut().shuffle_variables();

        self.delay_construct();
        self.time_component.set_solver_construction_time();

        for var in self.core_component.borrow().variables.iter() {
            let n = random::<usize>() % var.borrow().domain_size();
            // let _ = var.borrow_mut().assign_idx(n, self.core.level);
            self.decide_the_variable_with_idx(var, n);
        }
        self.solutions.record_solution(
            &self.core_component.borrow().variables,
            self.time_component.get_timer().get(),
        );
        for var in self.core_component.borrow().variables.iter() {
            var.borrow_mut()
                .restore_to_limit(self.core_component.borrow().level);
            let n = random::<usize>() % var.borrow().domain_size();

            self.decide_the_variable_with_idx(var, n);
        }
        self.solutions.record_solution(
            &self.core_component.borrow().variables,
            self.time_component.get_timer().get(),
        );
    }

    pub fn print_statistics(&self) {
        println!(
            "init time: {:?}",
            self.time_component.get_problem_set_time()
        );
        println!(
            "solver init time: {:?}",
            self.time_component.get_solver_construction_time()
        );
        println!("{}", self.solutions);
        println!(
            "solving time: {:?}",
            self.time_component.get_time_interval()
        );
    }
}

impl Display for Solver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
impl Hash for Solver {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        todo!()
    }
}
#[allow(dead_code)]
impl From<&Problem> for Solver {
    fn from(value: &Problem) -> Self {
        value.solver()
    }
}
