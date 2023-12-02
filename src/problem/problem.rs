/*
 * <p>@project_name: RuCos
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
use crate::solver::solver::solver::Solver;
use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::rc::Rc;

// pub struct Problem<X, C> where X: VariableTrait, C: ConstraintTrait {
pub struct Problem {
    name: String,
    variables: Box<Vec<Rc<RefCell<Variable>>>>,
    constraints: Box<Vec<Rc<RefCell<Box<dyn ConstraintTrait>>>>>,
    static_variables_id: i32,
    solver: Solver,
}

// impl<X, C> Problem<X, C> where X: VariableTrait, C: ConstraintTrait
impl Problem {
    pub fn new() -> Problem {
        Self {
            name: String::new(),
            variables: Box::new(vec![]),
            constraints: Box::new(vec![]),
            static_variables_id: 0,
            solver: Solver {},
        }
    }

    pub fn add_variable(&mut self, var: Variable) {
        self.variables.push(Rc::new(RefCell::new(var)))
    }

    pub fn add_constraint(&mut self, name: Box<dyn ConstraintTrait>) {
        self.constraints.push(Rc::new(RefCell::new(name)))
    }

    pub fn get_new_variable_id(&mut self) -> i32 {
        self.static_variables_id += 1;
        self.static_variables_id - 1
    }

    pub fn maximum_arity(&self) -> usize {
        todo!()
    }

    pub fn minimum_arity(&self) -> usize {
        todo!()
    }

    pub fn maximum_domain_size(&self) -> usize {
        todo!()
    }

    pub fn minimum_domain_size(&self) -> usize {
        todo!()
    }
    pub fn number_of_constraints(&self) -> usize {
        self.constraints.len()
    }

    pub fn number_of_variables(&self) -> usize {
        self.variables.len()
    }
}
