use crate::constraint::comparison::all_different::bound_consistency::BoundConsistency;
use std::cell::RefCell;
/* * *
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
 * * */
use crate::constraint::constraint::ConstraintTrait;
use crate::constraint::constraint_factory::XConstraintType;
use crate::constraint::propagator::PropagatorTrait;

use crate::solve::seal::Seal;
use crate::solve::solver::solver::InnerSolver;
use crate::variable::variable::Var;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct AllDifferent {
    solver: Option<InnerSolver>,
    scope: Vec<Var>,
    propagators: Vec<Rc<RefCell<dyn PropagatorTrait>>>,
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
        let propagators: Vec<Rc<RefCell<dyn PropagatorTrait>>> = vec![];
        Self {
            solver: None,
            scope,
            propagators,
            r#type: XConstraintType::XAllDifferent,
        }
    }

    pub fn new_with_reference(scope_ref: Vec<&Var>) -> Self {
        let mut scope = vec![];
        for &e in scope_ref.iter() {
            scope.push(e.clone())
        }

        let propagators: Vec<Rc<RefCell<dyn PropagatorTrait>>> = vec![];
        Self {
            solver: None,
            scope,
            propagators,
            r#type: XConstraintType::XAllDifferent,
        }
    }
}

#[allow(dead_code)]
impl ConstraintTrait for AllDifferent {
    fn get_propagators(&mut self) -> &Vec<Rc<RefCell<dyn PropagatorTrait>>> {
        &mut self.propagators
    }

    fn is_satisfied(&self) -> bool {
        todo!()
    }

    fn get_name(&self) -> &str {
        todo!()
    }

    fn restore_to_level(&mut self, _level: usize) {
        todo!()
    }

    fn delay_construct(&mut self, solver: &InnerSolver) {
        self.solver = Some(solver.clone());
        self.propagators
            .push(Rc::new(RefCell::new(BoundConsistency::new(&self.scope))));
        // solver.borrow().do_something_variables(|var|{
        //     print!("{} ",var.borrow().get_id());
        // });
        // println!();
        // println!("{:?}", self.scope)
    }

    fn get_type(&self) -> &XConstraintType {
        &self.r#type
    }

    fn get_scope(&self) -> &Vec<Var> {
        &self.scope
    }

    fn get_arity(&self) -> usize {
        self.scope.len()
    }
}
