/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/8 12:15
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::constraint::constraint::ConstraintTrait;
use crate::constraint::constraint_factory::XConstraintType;
use crate::constraint::genecric::extension::compact_table::CompactTable;
use crate::constraint::propagator::PropagatorTrait;
use crate::solve::solver::solver::Solver;
use crate::variable::variable::Var;

use std::fmt::{Display, Formatter};

use crate::solve::seal::Seal;

#[allow(dead_code)]
pub struct Extension {
    scope: Vec<Var>,
    solver: Option<Seal<Solver>>,
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
    pub fn new(scope: Vec<Var>) -> Self {
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

    fn delay_construct(&mut self, solver: Seal<Solver>) {
        self.solver = Some(solver);
        // println!("extension:")
    }

    fn get_type(&self) -> &XConstraintType {
        &self.r#type
    }

    fn get_scope(&self) -> &Vec<Var> {
        &self.scope
    }
}
