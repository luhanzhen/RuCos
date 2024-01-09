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

use crate::problem::problem::Problem;
use crate::solve::seal::Seal;
use crate::solve::solution::Solution;
use crate::solve::solver::branching_component::BranchingComponent;
use crate::solve::solver::callback_set::CallbackSet;
use crate::solve::solver::core_component::CoreComponent;
use crate::solve::solver::propagation_component::PropagationComponent;
use crate::solve::solver::status_component::*;
use crate::solve::solver::time_component::TimeComponent;

use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

pub type InnerSolver = Seal<CoreComponent>;
#[allow(dead_code)]
#[derive(Debug)]
pub struct Solver {
    solutions: Solution,
    status_component: StatusComponent,
    core_component: Seal<CoreComponent>,
    time_component: TimeComponent,
    branching_component: BranchingComponent,
    callback_set: CallbackSet,
    propagation_component: PropagationComponent,
}

impl Clone for Solver {
    fn clone(&self) -> Self {
        println!("Cloning Solver");
        Self {
            solutions: self.solutions.clone(),
            time_component: self.time_component.clone(),
            core_component: self.core_component.clone(),
            branching_component: self.branching_component.clone(),
            callback_set: CallbackSet::new(),
            status_component: self.status_component.clone(),

            propagation_component: self.propagation_component.clone(),
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
        let core_component = Seal::new(core);
        let heuristic_component = BranchingComponent::new(core_component.clone());
        let propagation_component = PropagationComponent::new(core_component.clone());
        Self {
            solutions,
            core_component,
            time_component: TimeComponent::new(problem.time()),
            branching_component: heuristic_component,
            callback_set: CallbackSet::new(),
            status_component: StatusComponent::new(),
            propagation_component,
        }
    }

    pub fn solve(&mut self) {
        self.time_component.reset();
        self.branching_component.choose_strategy();
        self.core_component.borrow_mut().shuffle_variables();

        self.propagation_component.delay_construct();
        self.time_component.set_solver_construction_time();
        self.propagation_component.propagate();

        for var in self.core_component.borrow().variables.iter() {
            self.branching_component.decide_var(var);
        }
        self.solutions.record_solution(
            &self.core_component.borrow().variables,
            self.time_component.get_timer().get(),
        );
        for var in self.core_component.borrow().variables.iter() {
            var.borrow_mut()
                .restore_to_limit(self.core_component.borrow().level);

            self.branching_component.decide_var(var);
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
