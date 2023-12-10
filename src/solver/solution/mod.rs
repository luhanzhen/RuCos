/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/10 18:13
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::utils::time_interval::TimeInterval;
use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::time::Duration;

pub struct Solution {
    variables: Vec<Rc<RefCell<Variable>>>,
    variable_index: Vec<i32>,
    solution: Vec<Vec<i32>>,
    solution_time: Vec<Duration>, //seconds
    solution_count: usize,
}

#[allow(dead_code)]
impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn new(variables_in: &Vec<Rc<RefCell<Variable>>>) -> Self {
        let mut variable_index = vec![];
        let mut variables = vec![];
        for var in variables_in.iter() {
            variable_index.push(var.borrow().get_id());
            variables.push(var.clone());
        }
        Self {
            variables,
            variable_index,
            solution: vec![],
            solution_time: vec![],
            solution_count: 0,
        }
    }

    pub fn record_solution(&mut self, timer: &TimeInterval) {
        self.solution_time.push(timer.get());
        let mut solution = vec![];
        for var in self.variables.iter() {
            match var.borrow().value() {
                None => {}
                Some(value) => {
                    solution.push(value);
                }
            }
        }
        self.solution.push(solution)
    }

    pub fn get_solution_counter(&self) -> usize {
        self.solution_count
    }
}
