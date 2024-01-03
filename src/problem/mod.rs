/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/2 13:21
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */
pub mod problem;

/// problem!() return the empty problem.
/// problem!(name:&str) return the  problem with name.
#[macro_export]
macro_rules! problem {
    () => {
        Problem::default();
    };

    ($name:expr) => {
        Problem::new_with_name($name);
    };
}
