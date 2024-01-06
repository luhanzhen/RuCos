/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/2 13:24
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */
use crate::exception::exception_factory::ExceptionFactory;
use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::ExceptionType;
use crate::problem::problem::Problem;
use crate::variable::domain::domain_trait::DomainTrait;
use crate::variable::domain::Domain;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::rc::Rc;

pub struct Var {
    cell: Rc<RefCell<Variable>>,
}

impl Eq for Var {}

impl Var {
    pub fn new(problem: &mut Problem, name: &str, dom: Domain) -> Self {
        Self {
            cell: Variable::new(problem, name, dom),
        }
    }

    pub fn new_without_problem(name: &str, dom: Domain) -> Self {
        Self {
            cell: Variable::new_without_problem(name, dom),
        }
    }

    fn new_with_rc_cell(cell: Rc<RefCell<Variable>>) -> Self {
        Self { cell }
    }

    #[inline]
    pub fn borrow(&self) -> Ref<'_, Variable> {
        self.cell.borrow()
    }
    #[inline]
    pub fn borrow_mut(&self) -> RefMut<'_, Variable> {
        self.cell.borrow_mut()
    }
}
impl Clone for Var {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            cell: Rc::clone(&self.cell),
        }
    }
}
impl Debug for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cell.borrow())
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cell.borrow())
    }
}

impl PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
}

impl Hash for Var {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cell.borrow().hash(state)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Variable {
    id: usize,
    name: String,
    // problem: Option<Rc<RefCell<Problem>>>,
    domain: Domain,
    empty_domain_exception: Box<dyn ExceptionTrait>,
    vale_not_found_exception: Box<dyn ExceptionTrait>,
}
impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]:{}", self.name, self.id, self.domain)
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[allow(dead_code)]
impl Variable {
    pub fn new(problem: &mut Problem, name: &str, dom: Domain) -> Rc<RefCell<Self>> {
        let var = Rc::new(RefCell::new(Self {
            id: problem.get_new_variable_id(),
            name: String::from(name),
            // problem: Some(Rc::new(RefCell::new(problem.clone()))),
            domain: dom,
            empty_domain_exception: ExceptionFactory::new(
                ExceptionType::EmptyDomainExceptionType,
                format!("{}'s domain is empty.", name).as_str(),
            ),

            vale_not_found_exception: ExceptionFactory::new(
                ExceptionType::ValueNotFoundExceptionType,
                format!("{}'s value is not found.", name).as_str(),
            ),
        }));
        problem.add_variable(Var::new_with_rc_cell(var.clone()));
        var
    }
    #[inline]
    pub fn get_id(&self) -> usize {
        self.id
    }
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.domain.is_empty()
    }

    pub fn new_without_problem(name: &str, dom: Domain) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id: usize::MAX,
            name: String::from(name),
            // problem: None,
            domain: dom,
            empty_domain_exception: ExceptionFactory::new(
                ExceptionType::EmptyDomainExceptionType,
                format!("{}'s domain is empty.", name).as_str(),
            ),

            vale_not_found_exception: ExceptionFactory::new(
                ExceptionType::ValueNotFoundExceptionType,
                format!("{}'s value is not found.", name).as_str(),
            ),
        }))
    }
    #[inline]
    pub(crate) fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub(crate) fn delete_value_at_level(
        &mut self,
        value: i32,
        level: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>> {
        if self.domain.is_limit_recorded_at_level(level) {
            self.domain.record_limit(level)
        }
        self.domain.delete_value_at_level(value, level);
        if self.domain.is_empty() {
            Err(&self.empty_domain_exception)
        } else {
            Ok(self.domain.size())
        }
    }

    pub(crate) fn value(&self) -> Option<i32> {
        if self.domain.size() != 1 {
            None
        } else {
            self.domain.values_at_position(0)
        }
    }
    /// if the idx is not in the domain, return None
    #[inline]
    pub fn value_to_idx(&self, value: i32) -> Option<usize> {
        self.domain.value_to_idx(value)
    }

    /// if the value is not in the domain, return None
    #[inline]
    pub fn idx_to_value(&self, idx: usize) -> Option<i32> {
        self.domain.idx_to_value(idx)
    }

    pub fn record_limit(&mut self, level: usize) {
        self.domain.record_limit(level)
    }
    pub(crate) fn the_idx_of_the_only_value(&self) -> Option<usize> {
        if self.domain.size() != 1 {
            None
        } else {
            Some(self.domain.first_idx())
        }
    }

    pub(crate) fn assign_value(
        &mut self,
        value: i32,
        level: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>> {
        if !self.domain.contains_value(value) {
            Err(&self.vale_not_found_exception)
        } else {
            let idx = self.domain.value_to_idx(value).unwrap();
            self.domain.reduce_to_idx(idx, level);
            Ok(idx)
        }
    }

    pub(crate) fn assign_idx(
        &mut self,
        idx: usize,
        level: usize,
    ) -> Result<i32, &Box<dyn ExceptionTrait>> {
        if !self.domain.contains_idx(idx) {
            Err(&self.vale_not_found_exception)
        } else {
            let value = self.domain.idx_to_value(idx).unwrap();
            self.domain.reduce_to_idx(idx, level);
            Ok(value)
        }
    }

    pub(crate) fn delete_idx_at_level(&mut self, idx: usize, level: usize) {
        if self.domain.is_limit_recorded_at_level(level) {
            self.domain.record_limit(level)
        }
        self.domain.delete_idx_at_level(idx, level);
        // if self.domain.is_empty() {
        //     Err(&self.empty_domain_exception)
        // } else {
        //     Ok(self.domain.size())
        // }
    }

    pub(crate) fn update_idx_upper_bound_at_level(
        &mut self,
        update_idx: usize,
        level: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>> {
        let size = self
            .domain
            .update_idx_upper_bound_at_level(update_idx, level);
        if size == 0 {
            Err(&self.empty_domain_exception)
        } else {
            Ok(size)
        }
    }

    pub(crate) fn update_idx_lower_bound_at_level(
        &mut self,
        update_idx: usize,
        level: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>> {
        let size = self
            .domain
            .update_idx_lower_bound_at_level(update_idx, level);
        if size == 0 {
            Err(&self.empty_domain_exception)
        } else {
            Ok(size)
        }
    }

    pub fn restore_to_limit(&mut self, level: usize) {
        self.domain.restore_to_limit(level)
    }

    #[inline]
    pub(crate) fn domain_size(&self) -> usize {
        self.domain.size()
    }

    #[inline]
    pub(crate) fn random_idx(&self) -> usize {
        self.domain.random_idx()
    }
    #[inline]
    pub(crate) fn random_value(&self) -> i32 {
        let n = self.domain.random_idx();
        if let Some(e) = self.domain.idx_to_value(n) {
            e
        } else {
            self.minimum_value()
        }
    }

    #[inline]
    pub(crate) fn domain_max_size(&self) -> usize {
        self.domain.max_size()
    }
    #[inline]
    pub(crate) fn contains_value(&self, value: i32) -> bool {
        self.domain.contains_value(value)
    }

    #[inline]
    pub(crate) fn contains_idx(&self, idx: usize) -> bool {
        self.domain.contains_idx(idx)
    }
    #[inline]
    pub fn maximum_value(&self) -> i32 {
        self.domain.maximum_value()
    }

    #[inline]
    pub fn minimum_idx(&self) -> usize {
        self.domain.minimum_idx()
    }

    #[inline]
    pub fn maximum_idx(&self) -> usize {
        self.domain.maximum_idx()
    }

    #[inline]
    pub fn minimum_value(&self) -> i32 {
        self.domain.minimum_value()
    }

    // pub fn get_empty_domain_exception(&self)-> &Box<dyn ExceptionTrait>
    // {
    //     &self.empty_domain_exception
    // }
}
impl Index<usize> for Variable {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.domain[index]
    }
}
