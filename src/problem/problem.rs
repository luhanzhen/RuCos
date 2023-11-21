/*
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/16 13:30
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::constraint::constraint::ConstraintTrait;
use crate::variable::variable::VariableTrait;
use std::cell::RefCell;
use std::rc::Rc;

// pub struct Problem<X, C> where X: VariableTrait, C: ConstraintTrait {
pub struct Problem {
    name: String,
    variables: Box<Vec<Rc<RefCell<Box<dyn VariableTrait>>>>>,
    constraints: Box<Vec<Rc<RefCell<Box<dyn ConstraintTrait>>>>>,
}

// impl<X, C> Problem<X, C> where X: VariableTrait, C: ConstraintTrait
impl Problem {
    pub fn new() -> Problem {
        Self {
            name: String::new(),
            variables: Box::new(vec![]),
            constraints: Box::new(vec![]),
        }
    }

    pub fn add_variable(&mut self, name: Box<dyn VariableTrait>) {
        self.variables.push(Rc::new(RefCell::new(name)))
    }

    pub fn add_constraint(&mut self, name: Box<dyn ConstraintTrait>) {
        self.constraints.push(Rc::new(RefCell::new(name)))
    }
}
