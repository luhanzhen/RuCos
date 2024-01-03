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
    fn should_restart(&mut self) -> bool;

    fn initialize(&mut self);

    // fn get_solver(&mut self) -> &mut Rc<RefCell<Solver>>;
}
