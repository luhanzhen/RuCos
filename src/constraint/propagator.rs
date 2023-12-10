/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/16 14:14
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::exception::exception_trait::ExceptionTrait;
use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::rc::Rc;

pub trait PropagatorTrait {
    fn initialise(&mut self);
    fn filter_by_variable(
        &mut self,
        dummy: &Rc<RefCell<Variable>>,
    ) -> Result<usize, &Box<dyn ExceptionTrait>>;
    fn filter_by_arc(
        &mut self,
        dummy: &Rc<RefCell<Variable>>,
        value: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>>;
    fn is_coarse_grained(&self) -> bool;
    fn is_fine_grained(&self) -> bool;
    fn restore_to_level(&mut self);
}
