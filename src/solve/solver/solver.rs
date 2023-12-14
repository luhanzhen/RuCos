/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/2 20:08
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::constraint::constraint::ConstraintTrait;
use crate::problem::problem::Problem;
use crate::solve::heuristics::value::heuristic_value::HeuristicValueTrait;
use crate::solve::heuristics::variable::heuristic_variable::HeuristicVariableTrait;
use crate::solve::restart::restart_trait::RestartTrait;
use crate::solve::solution::Solution;
use crate::solve::solver::status::*;
use crate::utils::time_interval::TimeInterval;
use crate::variable::variable::Variable;
use rand::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

#[allow(dead_code)]
pub struct Solver {
    problem: Rc<RefCell<Problem>>,
    variables: Vec<Rc<RefCell<Variable>>>,
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
struct Core {
    level: usize,
    decides: usize,
    conflicts: usize,
    propagations: usize,
    filter: usize,
}

#[allow(dead_code)]
impl Core {
    pub fn new() -> Self {
        Self {
            level: 0usize,
            decides: 0,
            conflicts: 0,
            propagations: 0,
            filter: 0,
        }
    }
}

#[allow(dead_code)]
impl Solver {
    pub fn new(problem: &Problem) -> Solver {
        let tmp_cons = problem.get_constraints().clone();
        let tmp_var = problem.get_all_variables().clone();
        Self {
            problem: Rc::new(RefCell::new(problem.clone())),
            timer: Default::default(),
            solutions: Solution::new(&tmp_var),
            option_self: None,
            variables: tmp_var,
            constraints: tmp_cons,
            status: SearchStates::Init,
            result: SearchResult::Init,
            init_time: None,
            core: Core::new(),
            restart: None,
            value_heuristic: None,
            variable_heuristic: None,
        }
    }
    pub fn delay_construct(&mut self) {
        match self.option_self {
            None => self.option_self = Some(Rc::new(RefCell::new(self.clone()))),
            Some(_) => {}
        }
        for e in self.constraints.iter_mut() {
            if let Some(op) = &self.option_self {
                e.borrow_mut().delay_construct(op.clone());
            }
        }
    }
    pub(crate) fn get_all_variables(&self) -> &Vec<Rc<RefCell<Variable>>> {
        &self.variables
    }
    pub fn solve(&mut self) {
        self.init_time = Some(self.problem.borrow_mut().time());
        self.timer.reset();
        self.shuffle_variables();

        self.delay_construct();

        for var in self.variables.iter() {
            let n = random::<usize>() % var.borrow().domain_size();
            let _ = var.borrow_mut().assign_idx(n, self.core.level);
        }
        self.solutions.record_solution(&self.variables, &self.timer);
        self.solutions.record_solution(&self.variables, &self.timer);
    }

    fn decide_the_variable_with_idx(&mut self, _var: &Rc<RefCell<Variable>>, _idx: usize) {
        let _= _var.borrow_mut().assign_idx(_idx,self.core.level);

    }

    fn propagate(&mut self) {}

    fn first_propagate(&mut self) {}

    fn backtrack_to_level(&mut self, _level: usize) {

    }
    fn backtrack(&mut self) {}

    pub fn print_statistics(&self) {
        println!("init time: {:?}", self.init_time.unwrap());
        println!("{}", self.solutions.to_string());
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
            core: Core::new(),
            restart: None,
            value_heuristic: None,
            variable_heuristic: None,
        }
    }
}
