/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/2 13:24
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::exception::exception_factory::ExceptionFactory;
use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::ExceptionType;
use crate::problem::problem::Problem;
use crate::variable::domain::domain_trait::DomainTrait;
use crate::variable::domain::Domain;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[allow(dead_code)]
pub struct Variable {
    id: i32,
    name: String,
    problem: Rc<RefCell<Problem>>,
    domain: Domain,
    empty_exception: Box<dyn ExceptionTrait>,
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]:{}", self.name, self.id, self.domain.to_string())
    }
}

#[allow(dead_code)]
impl Variable {
    pub fn new(problem: &Rc<RefCell<Problem>>, name: &str, dom: &Domain) -> Rc<RefCell<Variable>> {
        let var = Rc::new(RefCell::new(Self {
            id: problem.borrow_mut().get_new_variable_id(),
            name: String::from(name),
            problem: problem.clone(),
            domain: dom.clone(),
            empty_exception: ExceptionFactory::new(
                ExceptionType::EmptyDomainExceptionType,
                format!("{}'s domain is empty", name).as_str(),
            ),
        }));
        problem.borrow_mut().add_variable(var.clone());
        var
    }

    pub fn delete_value_at_level(
        &mut self,
        value: i32,
        level: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>> {
        if self.domain.is_limit_recorded_at_level(level) {
            self.domain.record_limit(level)
        }
        self.domain.delete_value_at_level(value, level);
        if self.domain.is_empty() {
            Err(&self.empty_exception)
        } else {
            Ok(self.domain.size())
        }
    }

    pub fn value(&self) -> Option<i32> {
        if self.domain.size() != 1 {
            None
        } else {
            self.domain.values_at_position(0)
        }
    }

    pub fn value_idx(&self) -> Option<usize> {
        if self.domain.size() != 1 {
            None
        } else {
            Some(self.domain[0])
        }
    }

    pub fn delete_idx_at_level(
        &mut self,
        idx: usize,
        level: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>> {
        if self.domain.is_limit_recorded_at_level(level) {
            self.domain.record_limit(level)
        }
        self.domain.delete_idx_at_level(idx, level);
        if self.domain.is_empty() {
            Err(&self.empty_exception)
        } else {
            Ok(self.domain.size())
        }
    }

    #[inline]
    pub fn domain_size(&self) -> usize {
        self.domain.size()
    }
    #[inline]
    pub fn contains_value(&self, value: i32) -> bool {
        self.domain.contains_value(value)
    }

    #[inline]
    pub fn contains_idx(&self, idx: usize) -> bool {
        self.domain.contains_idx(idx)
    }
    #[inline]
    pub fn maximum_value(&self) -> i32 {
        self.domain.maximum_value()
    }

    #[inline]
    fn minimum_idx(&self) -> usize {
        self.domain.minimum_idx()
    }

    #[inline]
    fn maximum_idx(&self) -> usize {
        self.domain.maximum_idx()
    }
    #[inline]
    pub fn minimum_value(&self) -> i32 {
        self.domain.minimum_value()
    }
}
