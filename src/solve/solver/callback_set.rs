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
pub struct CallbackSet<'a, ND, DD, DR, NC>
where
    ND: NewDecision,
    DD: NewDecision,
    DR: NewDecision,
    NC: NonConsistency,
{
    new_decision: Vec<&'a mut ND>,
    delete_decision: Vec<&'a mut DD>,
    domain_reduction: Vec<&'a mut DR>,
    non_consistency: Vec<&'a mut NC>,
}

#[allow(dead_code)]
impl<'a, ND, DD, DR, NC> CallbackSet<'a, ND, DD, DR, NC>
where
    ND: NewDecision,
    DD: NewDecision,
    DR: NewDecision,
    NC: NonConsistency,
{
    pub fn new() -> CallbackSet<'a, ND, DD, DR, NC> {
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
    pub fn new_decision(&mut self, callback: &'a mut ND) {
        self.new_decision.push(callback);
    }
    pub fn delete_decision(&mut self, callback: &'a mut DD) {
        self.delete_decision.push(callback);
    }
    pub fn domain_reduction(&mut self, callback: &'a mut DR) {
        self.domain_reduction.push(callback);
    }
    pub fn non_consistency(&mut self, callback: &'a mut NC) {
        self.non_consistency.push(callback);
    }
}
