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
use crate::exception::exception_factory::ExceptionFactory;
use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::ExceptionType;
use crate::solver::solver::solver::Solver;
use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

// pub struct Problem<X, C> where X: VariableTrait, C: ConstraintTrait {
pub struct Problem {
    name: String,
    variables: Vec<Rc<RefCell<Variable>>>,
    map_variables: HashMap<i32, Rc<RefCell<Variable>>>,
    constraints: Vec<Rc<RefCell<dyn ConstraintTrait>>>,
    static_variables_id: i32,
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Clone for Problem {
    fn clone(&self) -> Self {
        // let mut v: Vec<Rc<RefCell<Variable>>> = vec![];
        // for e in self.variables.iter() {
        //     v.push(e.clone())
        // }
        // let mut c: Vec<Rc<RefCell<dyn ConstraintTrait>>> = vec![];
        // for e in self.constraints.iter() {
        //     c.push(e.clone())
        // }
        Self {
            name: self.name.clone(),
            variables: self.variables.clone(),
            map_variables: self.map_variables.clone(),
            constraints: self.constraints.clone(),
            static_variables_id: self.static_variables_id,
        }
    }
}

impl Default for Problem {
    fn default() -> Self {
        Self::new()
    }
}

// impl<X, C> Problem<X, C> where X: VariableTrait, C: ConstraintTrait
#[allow(dead_code)]
impl Problem {
    pub fn get_variable_by_id(
        &mut self,
        key: i32,
    ) -> Result<&Rc<RefCell<Variable>>, Box<dyn ExceptionTrait>> {
        match self.map_variables.get(&key) {
            None => Err(ExceptionFactory::new(
                ExceptionType::InvalidVariableExceptionType,
                "the variable is not found.",
            )),
            Some(v) => Ok(v),
        }
        // if self.map_variables.contains_key(&key)
        // {
        //
        // }else {
        //     Err(Box::new(ExceptionFactory::new(ExceptionType::InvalidVariableExceptionType,"the variable is not found.")))
        // }
    }
    pub fn new() -> Problem {
        Self {
            name: String::new(),
            variables: vec![],
            map_variables: Default::default(),
            constraints: vec![],
            static_variables_id: 0,
        }
    }
    pub fn new_with_name(name: &str) -> Problem {
        Self {
            name: name.to_string(),
            variables: vec![],
            map_variables: Default::default(),
            constraints: vec![],
            static_variables_id: 0,
        }
    }

    pub fn get_constraints(&mut self) -> &mut Vec<Rc<RefCell<dyn ConstraintTrait>>> {
        &mut self.constraints
    }

    pub(crate) fn add_variable(&mut self, var: Rc<RefCell<Variable>>) {
        self.map_variables
            .insert(var.borrow().get_id(), var.clone());
        self.variables.push(var);
    }

    pub fn new_constraint(&mut self, cons: Rc<RefCell<dyn ConstraintTrait>>) {
        self.constraints.push(cons)
    }

    pub fn new_variable(&mut self, var: Rc<RefCell<Variable>>) {
        var.borrow_mut().set_id(self.get_new_variable_id());
        self.map_variables
            .insert(var.borrow().get_id(), var.clone());
        self.variables.push(var)
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

    pub(crate) fn get_all_variables(&self) -> &Vec<Rc<RefCell<Variable>>> {
        &self.variables
    }
}
