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
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::time::Duration;

pub struct Solution {
    variables: Vec<Rc<RefCell<Variable>>>,
    variable_index: HashMap<i32, usize>,
    solution: Vec<Vec<usize>>,
    solution_time: Vec<Duration>,
    //seconds
    solution_count: usize,
}

#[allow(dead_code)]
impl Display for Solution {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut str = String::new();

        for solution in self.solution.iter() {
            for (i, s) in solution.iter().enumerate() {
                str.push_str(&format!("{} ", *s))
            }
            str.push_str("\n");
        }
        write!(f, "Solution:\n{}\n", str)
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn new(variables_in: &Vec<Rc<RefCell<Variable>>>) -> Self {
        let mut variable_index = HashMap::new();
        let mut variables = vec![];
        for (i, var) in variables_in.iter().enumerate() {
            // variable_index.push(var.borrow().get_id());
            variable_index.insert(var.borrow().get_id(), i);
            variables.push(Rc::clone(var));
        }
        Self {
            variables,
            variable_index,
            solution: vec![],
            solution_time: vec![],
            solution_count: 0,
        }
    }

    pub fn record_solution(
        &mut self,
        variables_in: &Vec<Rc<RefCell<Variable>>>,
        timer: &TimeInterval,
    ) {
        let time = timer.get();
        let mut solution = vec![];
        solution.resize(self.variable_index.len(), 0);
        for var in variables_in.iter() {
            match var.borrow().value_idx() {
                None => {
                    return;
                }
                Some(value) => {
                    let idx = self.variable_index.get(&var.borrow().get_id()).unwrap();
                    solution[*idx] = value;
                }
            }
        }
        self.solution_time.push(time);
        self.solution.push(solution)
    }

    pub fn get_solution_counter(&self) -> usize {
        self.solution_count
    }
}
