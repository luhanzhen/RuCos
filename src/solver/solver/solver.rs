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
use std::ptr::NonNull;
use std::rc::Rc;
use std::marker::PhantomPinned;
use std::pin::Pin;


#[allow(dead_code)]
pub struct Solver {
    problem: Rc<RefCell<Problem>>,
    solver: Rc<RefCell<NonNull<Solver>>>,
    _pin: PhantomPinned,
    status: SearchStates,
    result: SearchResult,
}

#[allow(dead_code)]
impl Solver {
    pub fn new(problem: &Problem) -> Pin<Box<Solver>> {

        let tmp  = Self {
            problem: Rc::new(RefCell::new(problem.clone())),
            solver: Rc::new(RefCell::new(NonNull::dangling())),
            _pin: PhantomPinned,
            status: SearchStates::Init,
            result: SearchResult::Init,
        };
        let slice=  NonNull::from(&tmp);
        let mut boxed = Box::pin(tmp);

        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);

            Pin::get_unchecked_mut(mut_ref).solver.borrow_mut() = slice ;
        }
        boxed
    }
    fn delay_construct(&mut self) {
        // if let None =  self.solver {
        //   self.solver = Some(
        //       Rc::new(RefCell::new(self))
        //   )
        // }
        //
        // for &e in self.problem.borrow_mut().get_constraints().iter()
        // {
        //     let so = Rc::new(RefCell::new(self));
        //     e.borrow_mut().delay_construct(so);
        // }
    }
}
