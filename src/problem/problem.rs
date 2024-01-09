/* * *
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
 * * */
use crate::constraint::constraint::Constraint;
use crate::exception::exception_factory::ExceptionFactory;
use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::ExceptionType;
use crate::solve::solver::solver::Solver;
use crate::utils::time_interval::TimeInterval;
use crate::variable::variable::Var;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{AddAssign, Index};

use std::time::Duration;

// pub struct Problem<X, C> where X: VariableTrait, C: ConstraintTrait {
#[derive(Debug)]
pub struct Problem {
    name: String,
    variables: Vec<Var>,
    map_variables: HashMap<usize, Var>,
    map_name_variables: HashMap<String, Var>,
    constraints: Vec<Constraint>,
    static_variables_id: usize,
    timer: TimeInterval,
}
impl Index<&str> for Problem {
    type Output = Var;
    fn index(&self, index: &str) -> &Self::Output {
        match self.map_name_variables.get(index) {
            None => {
                panic!("wrong index for variable in the Problem!!!")
            }
            Some(var) => var,
        }
    }
}
impl Index<usize> for Problem {
    type Output = Var;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get_variable_by_id(index) {
            Ok(var) => var,
            Err(_) => {
                panic!("wrong index for variable in the Problem!!!")
            }
        }
    }
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

impl AddAssign<Var> for Problem {
    fn add_assign(&mut self, rhs: Var) {
        self.add_variable(rhs);
    }
}

impl AddAssign<Constraint> for Problem {
    fn add_assign(&mut self, rhs: Constraint) {
        self.add_constraint(rhs)
    }
}

impl Clone for Problem {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            variables: vec![],
            map_variables: Default::default(),
            map_name_variables: Default::default(),
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
    pub fn new() -> Problem {
        Self {
            name: String::new(),
            variables: vec![],
            map_variables: Default::default(),
            map_name_variables: Default::default(),
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
            map_name_variables: Default::default(),
            constraints: vec![],
            static_variables_id: 0,
            timer: Default::default(),
        }
    }

    pub fn add_variable(&mut self, var: Var) {
        if var.borrow().get_id() == usize::MAX {
            var.borrow_mut().set_id(self.get_new_variable_id());
        }
        self.map_variables
            .insert(var.borrow().get_id(), var.clone());
        self.map_name_variables
            .insert(String::from(var.borrow().get_name()), var.clone());
        self.variables.push(var);
    }

    pub fn add_constraint(&mut self, cons: Constraint) {
        self.constraints.push(cons)
    }

    pub fn new_variable(&mut self, var: Var) {
        var.borrow_mut().set_id(self.get_new_variable_id());
        self.map_variables
            .insert(var.borrow().get_id(), var.clone());
        self.map_name_variables
            .insert(String::from(var.borrow().get_name()), var.clone());
        self.variables.push(var)
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

    pub fn time(&self) -> Duration {
        self.timer.get()
    }
}
#[allow(dead_code)]
impl Problem {
    pub(crate) fn get_variable_by_id(&self, key: usize) -> Result<&Var, Box<dyn ExceptionTrait>> {
        match self.map_variables.get(&key) {
            None => Err(ExceptionFactory::new(
                ExceptionType::InvalidVariableExceptionType,
                "the variable is not found.",
            )),
            Some(v) => Ok(v),
        }
    }
    pub(crate) fn get_constraints(&self) -> &Vec<Constraint> {
        &self.constraints
    }

    pub(crate) fn get_all_variables(&self) -> &Vec<Var> {
        &self.variables
    }

    pub(crate) fn get_new_variable_id(&mut self) -> usize {
        self.static_variables_id += 1;
        self.static_variables_id - 1
    }
}
