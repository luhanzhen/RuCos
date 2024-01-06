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
use crate::constraint::constraint::Constraint;
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
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;


#[allow(dead_code)]
#[derive(Debug)]
pub struct InnerSolver {
    problem: Seal<Problem>,
    variables: Vec<Var>,
    constraints: Vec<Constraint>,
    solutions: Solution,
    status_component: StatusComponent,
    core_component: CoreComponent,
    time_component: TimeComponent,
    heuristic_component: HeuristicComponent,
    callback_set: CallbackSet,
    pub(crate) self_reference: Seal<NonNull<Pin<Box<InnerSolver>>>>,
    _pin: PhantomPinned,
}

#[allow(dead_code)]
impl InnerSolver {
    // fn delay_construct(&mut self) {
    //     for e in self.constraints.iter() {
    //         e.borrow_mut().delay_construct(&mut self);
    //     }
    // }
    fn get_all_variables(&self) -> &Vec<Var> {
        &self.variables
    }
    fn get_variables_size(&self) -> usize {
        self.variables.len()
    }
    fn choose_strategy(&mut self) {

            self.heuristic_component.choose_strategy()

    }

    fn decide_the_variable_with_idx(&self, var: &Var, idx: usize) {
        let _ = var.borrow_mut().assign_idx(idx, self.core_component.level);
    }

    fn propagate(&mut self) {}

    fn first_propagate(&mut self) {}

    fn backtrack_to_level(&mut self, _level: usize) {}
    fn backtrack(&mut self) {}

    fn shuffle_variables(&mut self) {
        for i in (1..=self.variables.len()).rev() {
            self.variables.swap(i - 1, random::<usize>() % i);
        }
    }
}

#[allow(dead_code)]
impl InnerSolver {
    pub(crate) fn get_conflicts(&self) -> usize {
        self.core_component.conflicts
    }

    pub(crate) fn get_future_vars(&self) -> &HashSet<Var> {
        &self.core_component.future_vars
    }

    pub(crate) fn get_past_vars(&self) -> &HashSet<Var> {
        &self.core_component.past_vars
    }

    pub(crate) fn get_level(&self) -> usize {
        self.core_component.level
    }
    pub(crate) fn maximum_arity(&self) -> usize {
        let mut max = usize::MIN;
        for con in self.constraints.iter() {
            let m = con.borrow().get_arity();
            if max < m {
                max = m;
            }
        }
        max
    }

    pub(crate) fn minimum_arity(&self) -> usize {
        let mut min = usize::MIN;
        for con in self.constraints.iter() {
            let m = con.borrow().get_arity();
            if min > m {
                min = m;
            }
        }
        min
    }

    pub(crate) fn maximum_domain_size(&self) -> usize {
        let mut max = usize::MIN;
        for var in self.variables.iter() {
            let m = var.borrow().domain_size();
            if max < m {
                max = m;
            }
        }
        max
    }

    pub(crate) fn minimum_domain_size(&self) -> usize {
        let mut min = usize::MAX;
        for var in self.variables.iter() {
            let m = var.borrow().domain_size();
            if min > m {
                min = m;
            }
        }
        min
    }
}
impl Clone for InnerSolver {
    fn clone(&self) -> Self {
        println!("Cloning Solver");
        Self {
            problem: self.problem.clone(),
            variables: self.variables.clone(),
            constraints: self.constraints.clone(),
            solutions: Solution::new(&self.variables),
            time_component: TimeComponent::new(),
            core_component: CoreComponent::new(&self.variables),
            heuristic_component: self.heuristic_component.clone(),
            callback_set: CallbackSet::new(),
            status_component: self.status_component.clone(),
            self_reference: Seal::new(NonNull::dangling()),
            _pin: PhantomPinned,
        }
    }
}

#[allow(dead_code)]
impl InnerSolver {
    pub fn new(problem: &Problem) -> Pin<Box<InnerSolver>> {
        let tmp_cons = problem.get_constraints().clone();
        let tmp_var = problem.get_all_variables().clone();
        let core = CoreComponent::new(&tmp_var);

        let res =Self {
            problem: Seal::new(problem.clone()),
            solutions: Solution::new(&tmp_var),
            self_reference: Seal::new(NonNull::dangling()),
            variables: tmp_var,
            constraints: tmp_cons,
            core_component: core,
            time_component: TimeComponent::new(),
            heuristic_component: HeuristicComponent::new(),
            callback_set: CallbackSet::new(),
            status_component: StatusComponent::new(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let reference = NonNull::from(&boxed);

        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).self_reference.replace( reference);
        }
        boxed
    }

    pub fn solve(&mut self) {
        self.time_component.init(self.problem.borrow_mut().time());
        self.choose_strategy();
        self.shuffle_variables();

        // self.delay_construct();

        for var in self.variables.iter() {
            let n = random::<usize>() % var.borrow().domain_size();
            // let _ = var.borrow_mut().assign_idx(n, self.core.level);
            self.decide_the_variable_with_idx(var, n);
        }
        self.solutions
            .record_solution(&self.variables, self.time_component.get_timer());
        // self.solutions.record_solution(&self.variables, &self.timer);
    }

    pub fn print_statistics(&self) {
        println!(
            "init time: {:?}",
            self.time_component.get_problem_set_time()
        );
        println!("{}", self.solutions);
        println!(
            "solving time: {:?}",
            self.time_component.get_time_interval()
        );
    }
}

impl Display for InnerSolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
impl Hash for InnerSolver {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        todo!()
    }
}
