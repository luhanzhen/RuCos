use crate::constraint::propagator::{PropagatorTrait};
use crate::solve::seal::Seal;
use crate::solve::solver::core_component::CoreComponent;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/***
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:   2024/1/9 14:34
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct PropagationComponent {
    core_component: Seal<CoreComponent>,
    queue: VecDeque<Rc<RefCell<dyn PropagatorTrait>>>,
}

impl Clone for PropagationComponent {
    fn clone(&self) -> Self {
        Self {
            core_component: self.core_component.clone(),
            queue: Default::default(),
        }
    }
}
impl PropagationComponent {
    pub(crate) fn propagate(&mut self) {
        for constraint in self.core_component.borrow().constraints.iter() {
            let mut borrow = constraint.borrow_mut();
            let propagations = borrow.get_propagators();
            for e in propagations.iter() {
                self.queue.push_front(Rc::clone(e));
            }
        }
        while !self.queue.is_empty() {
            if let Some(element) = self.queue.pop_back() {
                let borrow = element.borrow();

                match borrow.get_priority() {
                    _ => {
                        print!("priority ")
                    }
                }
            }
        }
    }
}
impl PropagationComponent {
    pub(crate) fn new(core_component: Seal<CoreComponent>) -> Self {
        Self {
            core_component,
            queue: Default::default(),
        }
    }
    pub(crate) fn delay_construct(&mut self) {
        self.core_component
            .borrow()
            .do_something_constraint(|c| c.borrow_mut().delay_construct(&self.core_component));
        self.core_component
            .borrow()
            .add_constraint_to_variable_scoped();
    }

    pub(crate) fn first_propagate(&mut self) {}
}
