use crate::prelude::Solver;
use crate::solve::seal::Seal;
use crate::solve::solver::core_component::CoreComponent;
use crate::solve::solver::solver::InnerSolver;
use std::fmt::Debug;

/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/7 16:16
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */

pub trait RestartTrait: Debug {
    fn should_restart(&mut self, solver: &InnerSolver) -> bool;

    fn initialize(&mut self, solver: &InnerSolver);

    // fn get_solver(&mut self) -> &mut Rc<RefCell<Solver>>;
}
