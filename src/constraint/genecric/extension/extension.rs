/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/8 12:15
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::constraint::constraint::ConstraintTrait;
use crate::constraint::propagator::PropagatorTrait;
use crate::solver::solver::solver::Solver;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[allow(dead_code)]
pub struct Extension {
    solver: Option<Rc<RefCell<Solver>>>,
    propagators: Vec<Box<dyn PropagatorTrait>>,
}

#[allow(dead_code)]
impl Display for Extension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Extension {
    pub fn new() {}
}

#[allow(dead_code)]
impl ConstraintTrait for Extension {
    fn get_propagators(&mut self) -> &mut Vec<Box<dyn PropagatorTrait>> {
        &mut self.propagators
    }

    fn is_satisfied(&self) -> bool {
        todo!()
    }

    fn get_name(&self) -> &str {
        todo!()
    }

    fn restore_to_level(&mut self) {
        todo!()
    }

    fn arity(&self) -> usize {
        todo!()
    }

    fn delay_construct(&mut self, solver: Rc<RefCell<Solver>>) {
        self.solver = Some(solver)
    }
}
