/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/10 16:07
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
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[allow(dead_code)]
pub struct AllDifferent {
    scope: Vec<Var>,
    solver: Option<Rc<RefCell<Solver>>>,
    propagators: Vec<Box<dyn PropagatorTrait>>,
    r#type: XConstraintType,
}

#[allow(dead_code)]
impl Display for AllDifferent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
#[allow(dead_code)]
impl AllDifferent {
    pub fn new(scope: Vec<Var>) -> Self {
        let propagators: Vec<Box<dyn PropagatorTrait>> = vec![Box::new(CompactTable::new(&scope))];
        Self {
            scope,
            solver: None,
            propagators,
            r#type: XConstraintType::XAllDifferent,
        }
    }
}

#[allow(dead_code)]
impl ConstraintTrait for AllDifferent {
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
        // for e in self.solver.as_ref().unwrap().borrow().get_all_variables().iter()
        // {
        //     print!("{} ",e.borrow().get_id());
        // }
        // println!();
    }

    fn get_type(&self) -> &XConstraintType {
        &self.r#type
    }

    fn get_scope(&self) -> &Vec<Var> {
        &self.scope
    }
}
