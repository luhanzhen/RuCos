/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/2 13:21
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

pub mod comparison;
pub mod connection;
pub mod constraint;
pub mod constraint_factory;
pub mod counting;
pub mod genecric;
pub mod language;
pub mod misceleanous;
pub mod packing_and_scheduling;
pub mod propagator;



#[macro_export]
macro_rules! constraint {
      (table $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Extension::new(temp_vec)
        }
    };
}
