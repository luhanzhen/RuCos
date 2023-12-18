use std::cell::RefCell;
use std::rc::Rc;
use crate::solve::restart::restart_trait::RestartTrait;
use crate::solve::solver::solver::Solver;

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:   2023/12/18 20:58
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
#[allow(dead_code)]
pub struct GeometricRestart {}


#[allow(dead_code)]
impl GeometricRestart{

}
#[allow(dead_code)]
impl RestartTrait for GeometricRestart
{
    fn should_restart(&mut self) -> bool {
        todo!()
    }

    fn initialize(&mut self) {
        todo!()
    }

    fn get_solver(&mut self) -> &mut Rc<RefCell<Solver>> {
        todo!()
    }
}