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

use crate::problem::problem::Problem;
use crate::solver::solver::status::*;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Solver {
    problem: Rc<RefCell<Problem>>,
    status: SearchStates,
    result: SearchResult,
}

#[allow(dead_code)]
impl Solver {
    pub fn new(problem: &Problem) -> Self {
        Self {
            problem: Rc::new(RefCell::new(problem.clone())),
            status: SearchStates::Init,
            result: SearchResult::Init,
        }
    }
}
