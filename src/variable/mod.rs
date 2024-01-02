/**
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
 */

/// var!(problem, name, domain) this is a type for the variable.
/// var!(problem, name, x=>y) this is a type for the variable.
/// var!(problem, name, v1,v2,v3....) this is a type for the variable.
/// var!(name, domain) this is a type for the variable.
/// var!(name, x=>y) this is a type for the variable.
/// var!(name, v1,v2,v3....) this is a type for the variable.
///
///
///
#[macro_export]
macro_rules! bool {
    () => {
        Var::new_without_problem("", Domain::new_with_range(0..2))
    };
    ($name:expr ) => {
        Var::new_without_problem($name, Domain::new_with_range(0..2))
    };
    ($problem:expr; $name:expr) => {
        Var::new($problem, $name, Domain::new_with_range(0..2))
    };
}

#[macro_export]
macro_rules! var {
    ( $problem:expr; $name:expr; $dom:expr  ) => {
        Var::new($problem, $name, $dom)
    };

     ( $problem:expr; $name:expr; $x:expr => $y:expr  ) => {
        Var::new($problem, $name, Domain::new_with_range($x..$y))
    };

     ( $( $x:expr ),* ) => {
             {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                Var::new_without_problem("", Domain::new_with_values(temp_vec))
            }

        };

    ( $name:expr; $( $x:expr ),* ) => {
             {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                Var::new_without_problem($name, Domain::new_with_values(temp_vec))
            }

        };

     ($problem:expr; $name:expr; $( $x:expr ),*  ) => {
         {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Var::new($problem, $name, Domain::new_with_values(temp_vec))
        }
    };


    ( $name:expr; $dom:expr  ) => {
        Var::new_without_problem($name, $dom)
    };


    ( $name:expr; $x:expr => $y:expr   ) => {
        Var::new_without_problem($name, Domain::new_with_range($x..$y))
    };

    ( $x:expr => $y:expr ) => {
        Var::new_without_problem("", Domain::new_with_range($x..$y))
    };





}

pub mod domain;
pub mod variable;
