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

use crate::solve::heuristics::value::heuristic_value::HeuristicValueTrait;
use crate::solve::heuristics::value::value_first::ValueFirst;
use crate::solve::heuristics::variable::heuristic_variable::HeuristicVariableTrait;
use crate::solve::restart::luby_restart::LubyRestart;
use crate::solve::restart::restart_trait::RestartTrait;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct HeuristicComponent {
    restart: Option<Box<dyn RestartTrait>>,
    value_heuristic: Option<Box<dyn HeuristicValueTrait>>,
    variable_heuristic: Option<Box<dyn HeuristicVariableTrait>>,
}

impl HeuristicComponent {
    pub(crate) fn new() -> Self {
        Self {
            restart: None,
            value_heuristic: None,
            variable_heuristic: None,
        }
    }
    pub(crate) fn choose_strategy(&mut self) {
        self.value_heuristic = Some(Box::new(ValueFirst::new()));

        self.restart = Some(Box::new(LubyRestart::new_with_solver_and_random_factor()))
    }
}

impl Clone for HeuristicComponent {
    fn clone(&self) -> Self {
        Self {
            restart: None,
            value_heuristic: None,
            variable_heuristic: None,
        }
    }
}
