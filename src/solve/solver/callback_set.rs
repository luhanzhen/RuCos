use crate::constraint::constraint::ConstraintTrait;
use crate::solve::callbacks::delete_decision::DeleteDecision;
use crate::solve::callbacks::domain_reduction::DomainReduction;
use crate::solve::callbacks::new_decision::NewDecision;
use crate::solve::callbacks::non_consistency::NonConsistency;
use crate::solve::solver::solver::Solver;
use crate::variable::variable::Var;
use std::cell::RefCell;
use std::rc::Rc;

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/24 00:19
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
pub struct CallbackSet {
    new_decision: Vec<Rc<RefCell<dyn NewDecision>>>,
    delete_decision: Vec<Rc<RefCell<dyn DeleteDecision>>>,
    domain_reduction: Vec<Rc<RefCell<dyn DomainReduction>>>,
    non_consistency: Vec<Rc<RefCell<dyn NonConsistency>>>,
}

#[allow(dead_code)]
impl CallbackSet {
    pub fn new() -> CallbackSet {
        Self {
            new_decision: vec![],
            delete_decision: vec![],
            domain_reduction: vec![],
            non_consistency: vec![],
        }
    }

    pub fn notify_new_decision(&self, var: &Var, solver: &Solver) {
        for e in self.new_decision.iter() {
            e.borrow_mut().new_decision_callback(var, solver)
        }
    }
    pub fn notify_delete_decision(&self, var: &Var, value_idx: usize, solver: &Solver) {
        for e in self.delete_decision.iter() {
            e.borrow_mut()
                .delete_decision_callback(var, value_idx, solver)
        }
    }
    pub fn notify_domain_reduction(&self, var: &Var, value_idx: usize, solver: &Solver) {
        for e in self.domain_reduction.iter() {
            e.borrow_mut()
                .domain_reduction_callback(var, value_idx, solver)
        }
    }
    pub fn notify_domain_assignment(&self, var: &Var, value_idx: usize, solver: &Solver) {
        for e in self.domain_reduction.iter() {
            e.borrow_mut()
                .domain_assignment_callback(var, value_idx, solver)
        }
    }
    pub fn notify_non_consistency(
        &self,
        cons: &Rc<RefCell<dyn ConstraintTrait>>,
        level: usize,
        solver: &Solver,
    ) {
        for e in self.non_consistency.iter() {
            e.borrow_mut().non_consistency_callback(cons, level, solver)
        }
    }

    pub fn new_decision(&mut self, callback: Rc<RefCell<dyn NewDecision>>) {
        self.new_decision.push(callback);
    }
    pub fn delete_decision(&mut self, callback: Rc<RefCell<dyn DeleteDecision>>) {
        self.delete_decision.push(callback);
    }
    pub fn domain_reduction(&mut self, callback: Rc<RefCell<dyn DomainReduction>>) {
        self.domain_reduction.push(callback);
    }
    pub fn non_consistency(&mut self, callback: Rc<RefCell<dyn NonConsistency>>) {
        self.non_consistency.push(callback);
    }
}
