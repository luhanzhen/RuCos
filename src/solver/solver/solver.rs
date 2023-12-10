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
use crate::solver::solution::Solution;
use crate::utils::time_interval::TimeInterval;
use crate::variable::variable::Variable;
use std::rc::Rc;



#[allow(dead_code)]
pub struct Solver {
    problem: Rc<RefCell<Problem>>,
    variables: Vec<Rc<RefCell<Variable>>>,
    timer: TimeInterval,
    status: SearchStates,
    result: SearchResult,
    solutions: Solution,
}

// solver: Rc<RefCell<NonNull<Solver>>>,
// _pin: PhantomPinned,

impl Clone for Solver {
    fn clone(&self) -> Self {
        Self {
            problem: self.problem.clone(),
            variables: self.variables.clone(),
            timer: Default::default(),
            status: self.status.clone(),
            result: self.result.clone(),
            solutions: Solution::new(&self.variables),
        }
    }
}
#[allow(dead_code)]
impl Solver {
    pub fn new(problem: &Problem) -> Solver {
        let tmp = problem.get_all_variables().clone();
        Self {
            problem: Rc::new(RefCell::new(problem.clone())),
            // solver: Rc::new(RefCell::new(NonNull::dangling())),
            // _pin: PhantomPinned,
            timer: Default::default(),
            solutions: Solution::new(&tmp),
            variables: tmp,
            status: SearchStates::Init,
            result: SearchResult::Init,
        }
        // let slice=  NonNull::from(&tmp);
        // let mut boxed = Box::pin(tmp);
        //
        // unsafe {
        //     let mut_ref = Pin::as_mut(&mut boxed);
        //
        //     Pin::get_unchecked_mut(mut_ref).solver.borrow_mut() = slice ;
        // }
        // boxed
    }
    pub fn delay_construct(&mut self) {
        for e in self.problem.borrow_mut().get_constraints().iter_mut() {
            let so = Rc::new(RefCell::new(self.clone()));
            e.borrow_mut().delay_construct(so.clone());
        }
    }

    pub fn solve(&mut self) {}
    fn decide(&mut self) {}

    fn propagate(&mut self) {}

    fn first_propagate(&mut self) {}
}
