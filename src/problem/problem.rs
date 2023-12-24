/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/16 13:30
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::constraint::constraint::ConstraintTrait;
use crate::exception::exception_factory::ExceptionFactory;
use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::ExceptionType;
use crate::solve::solver::solver::Solver;
use crate::utils::time_interval::TimeInterval;
use crate::variable::variable::Var;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use std::rc::Rc;
use std::time::Duration;

// pub struct Problem<X, C> where X: VariableTrait, C: ConstraintTrait {
// #[derive(Debug)]
pub struct Problem {
    name: String,
    variables: Vec<Var>,
    map_variables: HashMap<i32, Var>,
    constraints: Vec<Rc<RefCell<dyn ConstraintTrait>>>,
    static_variables_id: i32,
    timer: TimeInterval,
}
impl Hash for Problem {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        todo!()
    }
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
            variables: vec![],
            map_variables: Default::default(),
            constraints: vec![],
            static_variables_id: self.static_variables_id,
            timer: Default::default(),
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
    pub fn get_variable_by_id(&mut self, key: i32) -> Result<&Var, Box<dyn ExceptionTrait>> {
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
            timer: Default::default(),
        }
    }
    pub fn new_with_name(name: &str) -> Problem {
        Self {
            name: name.to_string(),
            variables: vec![],
            map_variables: Default::default(),
            constraints: vec![],
            static_variables_id: 0,
            timer: Default::default(),
        }
    }

    pub fn get_constraints(&self) -> &Vec<Rc<RefCell<dyn ConstraintTrait>>> {
        &self.constraints
    }

    pub(crate) fn add_variable(&mut self, var: Var) {
        self.map_variables
            .insert(var.borrow().get_id(), var.clone());
        self.variables.push(var);
    }

    pub fn new_constraint(&mut self, cons: Rc<RefCell<dyn ConstraintTrait>>) {
        self.constraints.push(cons)
    }

    pub fn new_variable(&mut self, var: Var) {
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
        Solver::new(self)
    }

    pub(crate) fn get_all_variables(&self) -> &Vec<Var> {
        &self.variables
    }

    pub fn time(&self) -> Duration {
        self.timer.get()
    }
}
