/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/16 13:21
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::utils::linked_set::LinkedSet;
use crate::variable::domain::domain_range::DomainRange;
use crate::variable::domain::domain_trait::DomainTrait;
use crate::variable::domain::domain_values::DomainValues;
use std::fmt::{Display, Formatter};
use std::ops::{Index, Range};

pub mod domain_range;
pub mod domain_trait;
pub mod domain_values;

/// you can generate the domain with following code:
///let a = domain![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33];
///domain![1=>33];
#[macro_export]
macro_rules! domain {
    ( $x:expr => $y:expr ) => {
        // $pattern
         Domain::new_with_range($x..$y)
        // match $pat
        // {
        //     l..r =>{
        //           Domain::new_with_range(l..r)
        //     }
        // }

    };
    // ( $( $pat:pat ), *) => {
    //      $(
    //             println!("{:?}",$pat)
    //         )*
    //
    // };
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Domain::new_with_values(temp_vec)
        }
    };

}
// #[macro_export]
// macro_rules! patterns {
//     ($($pat:pat_param)|+) => {
//         $( println!("pat_param: {}", stringify!($pat)); )+
//     };
// }
#[derive(Debug)]
pub enum Domain {
    DomRange(DomainRange),
    DomValues(DomainValues),
}

impl Domain {
    pub fn new_with_values(vals: Vec<i32>) -> Self {
        Domain::DomValues(DomainValues::new(vals))
    }

    pub fn new_with_range(range: Range<i32>) -> Self {
        Domain::DomRange(DomainRange::new(range))
    }
}

// #[macro_export]
// macro_rules! repeat {
//     ($name:ident, $output:ident) => {
//         fn $name (&self) -> $output
//         {
//             match self {
//                 Domain::DomRange(r) => Domain::DomRange(r.$name ()),
//                 Domain::DomValues(v) => Domain::DomValues(v.$name ()),
//              }
//         }
//     };
//
//     ($name:ident, $input:ident, $in_type:ty, $output:ident) => {
//         fn $name (&self,$input: $in_type) -> $output
//         {
//             match self {
//                 Domain::DomRange(r) => Domain::DomRange(r.$name ()),
//                 Domain::DomValues(v) => Domain::DomValues(v.$name ()),
//              }
//         }
//     };
// }

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Domain::DomRange(r) => r.fmt(f),
            Domain::DomValues(v) => v.fmt(f),
        }
    }
}

impl Clone for Domain {
    fn clone(&self) -> Self {
        match self {
            Domain::DomRange(r) => Domain::DomRange(r.clone()),
            Domain::DomValues(v) => Domain::DomValues(v.clone()),
        }
    }
}

impl PartialEq for Domain {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Domain::DomRange(r) => match other {
                Domain::DomRange(or) => r.eq(or),
                _ => false,
            },
            Domain::DomValues(v) => match other {
                Domain::DomValues(ov) => v.eq(ov),
                _ => false,
            },
        }
    }
}

impl Index<usize> for Domain {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Domain::DomRange(r) => &r.get_elements()[index],
            Domain::DomValues(v) => &v.get_elements()[index],
        }
    }
}
impl DomainTrait for Domain {
    fn value_to_idx(&self, value: i32) -> Option<usize> {
        match self {
            Domain::DomRange(r) => r.value_to_idx(value),
            Domain::DomValues(v) => v.value_to_idx(value),
        }
    }

    fn idx_to_value(&self, idx: usize) -> Option<i32> {
        match self {
            Domain::DomRange(r) => r.idx_to_value(idx),
            Domain::DomValues(v) => v.idx_to_value(idx),
        }
    }

    fn is_idx_correspond_to_values(&self) -> bool {
        match self {
            Domain::DomRange(r) => r.is_idx_correspond_to_values(),
            Domain::DomValues(v) => v.is_idx_correspond_to_values(),
        }
    }
    // repeat! {hash, usize}
    fn hash(&self) -> usize {
        match self {
            Domain::DomRange(r) => r.hash(),
            Domain::DomValues(v) => v.hash(),
        }
    }
    fn get_elements(&self) -> &LinkedSet {
        match self {
            Domain::DomRange(r) => r.get_elements(),
            Domain::DomValues(v) => v.get_elements(),
        }
    }

    fn get_elements_mut(&mut self) -> &mut LinkedSet {
        match self {
            Domain::DomRange(r) => r.get_elements_mut(),
            Domain::DomValues(v) => v.get_elements_mut(),
        }
    }
}
