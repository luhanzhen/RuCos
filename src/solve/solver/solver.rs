/**
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
 */
use crate::constraint::constraint::ConstraintTrait;
use crate::problem::problem::Problem;
use crate::solve::heuristics::value::heuristic_value::HeuristicValueTrait;
use crate::solve::heuristics::value::value_first::ValueFirst;
use crate::solve::heuristics::variable::heuristic_variable::HeuristicVariableTrait;
use crate::solve::restart::luby_restart::LubyRestart;
use crate::solve::restart::restart_trait::RestartTrait;
use crate::solve::solution::Solution;
use crate::solve::solver::core::Core;
use crate::solve::solver::status::*;
use crate::utils::time_interval::TimeInterval;
use crate::variable::variable::{Var, Variable};
use rand::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

#[allow(dead_code)]
pub struct Solver {
    problem: Rc<RefCell<Problem>>,
    variables: Vec<Var>,
    constraints: Vec<Rc<RefCell<dyn ConstraintTrait>>>,
    timer: TimeInterval,
    status: SearchStates,
    result: SearchResult,
    solutions: Solution,
    option_self: Option<Rc<RefCell<Solver>>>,
    init_time: Option<Duration>,
    core: Core,
    restart: Option<Box<dyn RestartTrait>>,
    value_heuristic: Option<Box<dyn HeuristicValueTrait>>,
    variable_heuristic: Option<Box<dyn HeuristicVariableTrait>>,
}

#[allow(dead_code)]
impl From<&Problem> for Solver {
    fn from(value: &Problem) -> Self {
        value.solver()
    }
}

#[allow(dead_code)]
impl Solver {
    pub fn new(problem: &Problem) -> Solver {
        let tmp_cons = problem.get_constraints().clone();
        let tmp_var = problem.get_all_variables().clone();
        let core = Core::new(&tmp_var);
        let mut ret = Self {
            problem: Rc::new(RefCell::new(problem.clone())),
            timer: Default::default(),
            solutions: Solution::new(&tmp_var),
            option_self: None,
            variables: tmp_var,
            constraints: tmp_cons,
            status: SearchStates::Init,
            result: SearchResult::Unknown,
            init_time: None,
            core,
            restart: None,
            value_heuristic: None,
            variable_heuristic: None,
        };
        match ret.option_self {
            None => ret.option_self = Some(Rc::new(RefCell::new(ret.clone()))),
            Some(_) => {}
        }
        ret
    }

    pub fn get_conflicts(&self) -> usize {
        self.core.conflicts
    }
    pub fn delay_construct(&mut self) {
        for e in self.constraints.iter_mut() {
            if let Some(op) = &self.option_self {
                e.borrow_mut().delay_construct(op.clone());
            }
        }
    }
    pub(crate) fn get_all_variables(&self) -> &Vec<Var> {
        &self.variables
    }

    fn choose_strategy(&mut self) {
        self.value_heuristic = Some(Box::new(ValueFirst::new()));
        if let Some(value) = &self.option_self {
            self.restart = Some(Box::new(LubyRestart::new_with_solver_and_random_factor(
                value,
            )))
        }
    }
    pub fn solve(&mut self) {
        self.init_time = Some(self.problem.borrow_mut().time());
        self.timer.reset();
        self.choose_strategy();
        self.shuffle_variables();

        self.delay_construct();

        for var in self.variables.iter() {
            let n = random::<usize>() % var.borrow().domain_size();
            let _ = var.borrow_mut().assign_idx(n, self.core.level);
        }
        self.solutions.record_solution(&self.variables, &self.timer);
        // self.solutions.record_solution(&self.variables, &self.timer);
    }

    fn decide_the_variable_with_idx(&mut self, _var: &Rc<RefCell<Variable>>, _idx: usize) {
        let _ = _var.borrow_mut().assign_idx(_idx, self.core.level);
    }

    fn propagate(&mut self) {}

    fn first_propagate(&mut self) {}

    fn backtrack_to_level(&mut self, _level: usize) {}
    fn backtrack(&mut self) {}

    pub fn print_statistics(&self) {
        println!("init time: {:?}", self.init_time.unwrap());
        println!("{}", self.solutions);
        println!("solving time: {:?}", self.timer.get());
    }

    fn shuffle_variables(&mut self) {
        for i in (1..=self.variables.len()).rev() {
            self.variables.swap(i - 1, random::<usize>() % i);
        }
    }
}

impl Clone for Solver {
    fn clone(&self) -> Self {
        Self {
            problem: Rc::clone(&self.problem),
            variables: self.variables.clone(),
            constraints: self.constraints.clone(),
            timer: Default::default(),
            status: self.status.clone(),
            result: self.result.clone(),
            solutions: Solution::new(&self.variables),
            option_self: None,
            init_time: None,
            core: Core::new(&self.variables),
            restart: None,
            value_heuristic: None,
            variable_heuristic: None,
        }
    }
}
