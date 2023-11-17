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
use std::rc::Rc;
use std::sync::Arc;

// pub struct Problem<X, C> where X: VariableTrait, C: ConstraintTrait {
pub struct Problem {
    name: String,
    variables: Box<Vec<Rc<Arc<dyn VariableTrait>>>>,
    constraints: Box<Vec<Rc<Arc<dyn ConstraintTrait>>>>,
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
}
