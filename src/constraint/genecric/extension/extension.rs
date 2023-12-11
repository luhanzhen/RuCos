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
use crate::constraint::constraint_factory::XConstraintType;
use crate::constraint::genecric::extension::compact_table::CompactTable;
use crate::constraint::propagator::PropagatorTrait;
use crate::solve::solver::solver::Solver;
use crate::variable::variable::Variable;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[allow(dead_code)]
pub struct Extension {
    scope: Vec<Rc<RefCell<Variable>>>,
    solver: Option<Rc<RefCell<Solver>>>,
    propagators: Vec<Box<dyn PropagatorTrait>>,
    r#type: XConstraintType,
}

#[allow(dead_code)]
impl Display for Extension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Extension {
    pub fn new(scope: Vec<Rc<RefCell<Variable>>>) -> Self {
        let propagators: Vec<Box<dyn PropagatorTrait>> = vec![Box::new(CompactTable::new(&scope))];
        Self {
            scope,
            solver: None,
            propagators,
            r#type: XConstraintType::XExtension,
        }
    }
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
        self.solver = Some(solver);
        // println!("extension:")
    }

    fn get_type(&self) -> &XConstraintType {
        &self.r#type
    }

    fn get_scope(&self) -> &Vec<Rc<RefCell<Variable>>> {
        &self.scope
    }
}
