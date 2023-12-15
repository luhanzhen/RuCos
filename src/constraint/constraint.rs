/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/3 15:52
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::constraint::constraint_factory::XConstraintType;
use crate::constraint::propagator::PropagatorTrait;
use crate::solve::solver::solver::Solver;
use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub trait ConstraintTrait: Display {
    fn get_propagators(&mut self) -> &mut Vec<Box<dyn PropagatorTrait>>;

    fn is_satisfied(&self) -> bool;

    fn get_name(&self) -> &str;

    fn restore_to_level(&mut self);

    fn arity(&self) -> usize;

    fn delay_construct(&mut self, solver: Rc<RefCell<Solver>>);

    fn get_type(&self) -> &XConstraintType;

    fn get_scope(&self) -> &Vec<Rc<RefCell<Variable>>>;
}
