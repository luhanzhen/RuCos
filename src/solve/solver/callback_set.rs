use crate::constraint::constraint::Constraint;
use crate::solve::callbacks::delete_decision::DeleteDecision;
use crate::solve::callbacks::domain_reduction::DomainReduction;
use crate::solve::callbacks::new_decision::NewDecision;
use crate::solve::callbacks::non_consistency::NonConsistency;
use crate::solve::solver::solver::Solver;
use crate::variable::variable::Var;

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
    new_decision: Vec<Box<dyn NewDecision>>,
    delete_decision: Vec<Box<dyn DeleteDecision>>,
    domain_reduction: Vec<Box<dyn DomainReduction>>,
    non_consistency: Vec<Box<dyn NonConsistency>>,
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

    pub fn notify_new_decision(&mut self, var: &Var, solver: &Solver) {
        for e in self.new_decision.iter_mut() {
            e.new_decision_callback(var, solver)
        }
    }
    pub fn notify_delete_decision(&mut self, var: &Var, value_idx: usize, solver: &Solver) {
        for e in self.delete_decision.iter_mut() {
            e.delete_decision_callback(var, value_idx, solver)
        }
    }
    pub fn notify_domain_reduction(&mut self, var: &Var, value_idx: usize, solver: &Solver) {
        for e in self.domain_reduction.iter_mut() {
            e.domain_reduction_callback(var, value_idx, solver)
        }
    }
    pub fn notify_domain_assignment(&mut self, var: &Var, value_idx: usize, solver: &Solver) {
        for e in self.domain_reduction.iter_mut() {
            e.domain_assignment_callback(var, value_idx, solver)
        }
    }
    pub fn notify_non_consistency(&mut self, cons: &Constraint, level: usize, solver: &Solver) {
        for e in self.non_consistency.iter_mut() {
            e.non_consistency_callback(cons, level, solver)
        }
    }

    pub fn new_decision(&mut self, callback: Box<dyn NewDecision>) {
        self.new_decision.push(callback);
    }
    pub fn delete_decision(&mut self, callback: Box<dyn DeleteDecision>) {
        self.delete_decision.push(callback);
    }
    pub fn domain_reduction(&mut self, callback: Box<dyn DomainReduction>) {
        self.domain_reduction.push(callback);
    }
    pub fn non_consistency(&mut self, callback: Box<dyn NonConsistency>) {
        self.non_consistency.push(callback);
    }
}
