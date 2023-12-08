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
use std::fmt::{Display, Formatter};
use std::rc::Rc;

// pub struct Problem<X, C> where X: VariableTrait, C: ConstraintTrait {
pub struct Problem {
    name: String,
    variables: Vec<Rc<RefCell<Variable>>>,
    constraints: Vec<Rc<RefCell<dyn ConstraintTrait>>>,
    static_variables_id: i32,
    // solver: Solver,
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Clone for Problem {
    fn clone(&self) -> Self {
        let mut v: Vec<Rc<RefCell<Variable>>> = vec![];
        for e in self.variables.iter() {
            v.push(e.clone())
        }
        let mut c: Vec<Rc<RefCell<dyn ConstraintTrait>>> = vec![];
        for e in self.constraints.iter() {
            c.push(e.clone())
        }
        Self {
            name: self.name.clone(),
            variables: v,
            constraints: c,
            static_variables_id: self.static_variables_id,
        }
    }
}

// impl<X, C> Problem<X, C> where X: VariableTrait, C: ConstraintTrait
#[allow(dead_code)]
impl Problem {
    pub fn new() -> Rc<RefCell<Problem>> {
        Rc::new(RefCell::new(Self {
            name: String::new(),
            variables: vec![],
            constraints: vec![],
            static_variables_id: 0,
            // solver: Solver::new(),
        }))
    }

    pub fn get_constraints(&mut self) -> &mut Vec<Rc<RefCell<dyn ConstraintTrait>>> {
        &mut self.constraints
    }

    pub fn add_variable(&mut self, var: Rc<RefCell<Variable>>) {
        self.variables.push(var)
    }

    pub fn add_constraint(&mut self, cons: Rc<RefCell<dyn ConstraintTrait>>) {
        self.constraints.push(cons)
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
        let mut max = usize::MIN;
        for var in self.variables.iter() {
            let m = var.borrow().domain_size();
            if max < m {
                max = m;
            }
        }
        max
    }

    pub fn minimum_domain_size(&self) -> usize {
        let mut min = usize::MAX;
        for var in self.variables.iter() {
            let m = var.borrow().domain_size();
            if min > m {
                min = m;
            }
        }
        min
    }
    pub fn number_of_constraints(&self) -> usize {
        self.constraints.len()
    }

    pub fn number_of_variables(&self) -> usize {
        self.variables.len()
    }

    pub fn solver(&self) -> Solver {
        Solver::new(&self)
    }
}
