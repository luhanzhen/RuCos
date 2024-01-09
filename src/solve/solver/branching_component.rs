/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/1/5 23:35
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */

use crate::prelude::Var;
use crate::solve::heuristics::value::heuristic_value_trait::HeuristicValueTrait;
use crate::solve::heuristics::value::value_first::ValueFirst;
use crate::solve::heuristics::variable::heuristic_variable_trait::HeuristicVariableTrait;
use crate::solve::restart::luby_restart::LubyRestart;
use crate::solve::restart::restart_trait::RestartTrait;
use crate::solve::seal::Seal;
use crate::solve::solver::core_component::CoreComponent;
use rand::random;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct BranchingComponent {
    core_component: Seal<CoreComponent>,
    restart: Option<Box<dyn RestartTrait>>,
    value_heuristic: Option<Box<dyn HeuristicValueTrait>>,
    variable_heuristic: Option<Box<dyn HeuristicVariableTrait>>,
}
#[allow(dead_code)]
impl BranchingComponent {
    pub(crate) fn new(core_component: Seal<CoreComponent>) -> Self {
        Self {
            core_component,
            restart: None,
            value_heuristic: None,
            variable_heuristic: None,
        }
    }

    pub(crate) fn backtrack_to_level(&mut self, _level: usize) {}
    pub(crate) fn backtrack(&mut self) {}

    pub(crate) fn decide(&mut self) {
        let level = self.core_component.borrow().level;
        let binding = self.core_component.borrow_mut();
        let var = binding.get_all_variables().get(level).unwrap();
        let idx = random::<usize>() % var.borrow().domain_size();
        var.borrow_mut()
            .assign_idx(idx, level)
            .expect("TODO: panic message");
        var.borrow_mut().record_limit(level);
    }

    pub(crate) fn decide_var(&mut self, var: &Var) {
        let level = self.core_component.borrow().level;
        let idx = random::<usize>() % var.borrow().domain_size();
        let _ = var.borrow_mut().assign_idx(idx, level);
        var.borrow_mut().record_limit(level);
    }
    pub(crate) fn choose_strategy(&mut self) {
        self.value_heuristic = Some(Box::new(ValueFirst::new()));

        self.restart = Some(Box::new(LubyRestart::new_with_solver_and_random_factor()))
    }
}

impl Clone for BranchingComponent {
    fn clone(&self) -> Self {
        Self {
            core_component: self.core_component.clone(),
            restart: None,
            value_heuristic: None,
            variable_heuristic: None,
        }
    }
}
