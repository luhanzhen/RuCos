use crate::constraint::comparison::all_different::all_different::AllDifferent;
/* * *
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
 * * */
use crate::constraint::constraint_factory::XConstraintType;
use crate::constraint::genecric::extension::extension::Extension;
use crate::constraint::propagator::PropagatorTrait;
use crate::solve::seal::Seal;
use crate::solve::solver::solver::Solver;
use crate::variable::variable::Var;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub struct Constraint {
    cell: Rc<RefCell<dyn ConstraintTrait>>,
}
#[allow(dead_code)]
impl Constraint {
    pub fn new_all_different(scope: Vec<Var>) -> Self {
        Self {
            cell: Rc::new(RefCell::new(AllDifferent::new(scope))),
        }
    }
    pub fn new_all_different_with_reference(scope: Vec<&Var>) -> Self {
        Self {
            cell: Rc::new(RefCell::new(AllDifferent::new_with_reference(scope))),
        }
    }

    pub fn new_extension(scope: Vec<Var>) -> Self {
        Self {
            cell: Rc::new(RefCell::new(Extension::new(scope))),
        }
    }
    pub(crate) fn new_with_rc_cell(cell: Rc<RefCell<dyn ConstraintTrait>>) -> Self {
        Self { cell }
    }
    #[inline]
    pub(crate) fn borrow(&self) -> Ref<'_, dyn ConstraintTrait> {
        self.cell.borrow()
    }
    #[inline]
    pub(crate) fn borrow_mut(&self) -> RefMut<'_, dyn ConstraintTrait> {
        self.cell.borrow_mut()
    }
}
impl Clone for Constraint {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            cell: Rc::clone(&self.cell),
        }
    }
}
impl Display for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cell.borrow())
    }
}

pub(crate) trait ConstraintTrait: Display + Debug {
    fn get_propagators(&mut self) -> &mut Vec<Box<dyn PropagatorTrait>>;

    fn is_satisfied(&self) -> bool;

    fn get_name(&self) -> &str;

    fn restore_to_level(&mut self, level: usize);

    fn delay_construct(&mut self, solver: Seal<Solver>);

    fn get_type(&self) -> &XConstraintType;

    fn get_scope(&self) -> &Vec<Var>;

    fn get_arity(&self) -> usize;
}
