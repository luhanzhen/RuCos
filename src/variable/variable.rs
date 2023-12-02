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

use crate::problem::problem::Problem;

use crate::variable::domain::Domain;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub trait VariableTrait {}

pub struct Variable {
    id: i32,
    name: String,
    problem: Rc<RefCell<Problem>>,
    domain: Rc<RefCell<Domain>>,
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}]:{}",
            self.name,
            self.id,
            self.domain.borrow().to_string()
        )
    }
}
impl Variable {
    pub fn new(problem: &Rc<RefCell<Problem>>, name: &str, dom: Domain) -> Self {
        Self {
            id: problem.borrow_mut().get_new_variable_id(),
            name: String::from(name),
            problem: problem.clone(),
            domain: Rc::new(RefCell::new(dom)),
        }
    }
}
