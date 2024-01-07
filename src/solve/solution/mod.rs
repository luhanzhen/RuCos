/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/10 18:13
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */

use crate::variable::variable::Var;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::time::Duration;
#[derive(Debug)]
pub struct Solution {
    variables: Vec<Var>,
    variable_index: HashMap<usize, usize>,
    solution: Vec<Vec<usize>>,
    solution_time: Vec<Duration>,
    //seconds
    solution_count: usize,
}

#[allow(dead_code)]
impl Display for Solution {
    // fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    //     let mut str = String::new();
    //
    //     for (ith, solution) in self.solution.iter().enumerate() {
    //         // str.push('\t');
    //         for (i, s) in solution.iter().enumerate() {
    //             str.push_str(&format!(
    //                 "{}={}\t",
    //                 self.variables[i].borrow().get_name(),
    //                 *s
    //             ))
    //         }
    //         str.push_str(&format!("\nsolution cost:{:?}\n", &self.solution_time[ith]));
    //     }
    //     write!(f, "Solution:\n{}", str)
    // }
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.solution.len() == 0 {
            write!(f, "No Solution:\n")
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic);
            let mut header = vec![String::from("time")];

            for (i, _) in self.solution[0].iter().enumerate() {
                header.push(self.variables[i].borrow().get_name().to_string())
            }

            table.set_header(header);
            for (ith, solution) in self.solution.iter().enumerate() {
                let mut row = vec![format!("{:?}", &self.solution_time[ith])];
                for s in solution.iter() {
                    row.push(format!("{}", *s))
                }

                table.add_row(row);
            }
            write!(f, "Solution:\n{}", table.to_string())
        }
    }
}
impl Clone for Solution {
    fn clone(&self) -> Self {
        Self {
            variables: self.variables.clone(),
            variable_index: self.variable_index.clone(),
            solution: vec![],
            solution_time: vec![],
            solution_count: 0,
        }
    }
}
#[allow(dead_code)]
impl Solution {
    pub(crate) fn new(variables_in: &Vec<Var>) -> Self {
        let mut variable_index = HashMap::new();
        let mut variables = vec![];
        for (i, var) in variables_in.iter().enumerate() {
            // variable_index.push(var.borrow().get_id());
            variable_index.insert(var.borrow().get_id(), i);
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

    pub(crate) fn record_solution(&mut self, variables_in: &Vec<Var>, time: Duration) {
        let mut solution = vec![];
        solution.resize(self.variable_index.len(), 0);
        for var in variables_in.iter() {
            match var.borrow().the_idx_of_the_only_value() {
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
