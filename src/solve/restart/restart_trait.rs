use std::fmt::Debug;
use crate::prelude::Solver;

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
    fn should_restart(&mut self,solver: &mut Solver) -> bool;

    fn initialize(&mut self,solver: &mut Solver);

    // fn get_solver(&mut self) -> &mut Rc<RefCell<Solver>>;
}
