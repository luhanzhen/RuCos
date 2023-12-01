/*
 * <p>@project_name: constraint_solver
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
use crate::variable::domain::domain_trait::DomainTrait;
use std::cell::RefCell;
use std::rc::Rc;

pub trait VariableTrait {}

pub struct Variable {
    id: i32,
    problem: Rc<RefCell<Problem>>,
    domain: Box<dyn DomainTrait>,
}
