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

use crate::utils::linked_set::{LinkedSet, LinkedSetIter};
use crate::variable::domain::domain_range::DomainRange;
use crate::variable::domain::domain_trait::DomainTrait;
use crate::variable::domain::domain_values::DomainValues;
use std::fmt::{Display, Formatter};
use std::ops::Range;

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

    fn delete_idx(&mut self, idx: usize, level: usize) {
        match self {
            Domain::DomRange(r) => r.delete_idx(idx, level),
            Domain::DomValues(v) => v.delete_idx(idx, level),
        }
    }

    fn delete_value(&mut self, value: i32, level: usize) {
        match self {
            Domain::DomRange(r) => r.delete_value(value, level),
            Domain::DomValues(v) => v.delete_value(value, level),
        }
    }

    fn which_level_deleted_value(&self, value: i32) -> Option<usize> {
        match self {
            Domain::DomRange(r) => r.which_level_deleted_value(value),
            Domain::DomValues(v) => v.which_level_deleted_value(value),
        }
    }
    fn which_level_deleted_idx(&self, idx: usize) -> Option<usize> {
        match self {
            Domain::DomRange(r) => r.which_level_deleted_idx(idx),
            Domain::DomValues(v) => v.which_level_deleted_idx(idx),
        }
    }
    fn record_limit(&mut self, level: usize) {
        match self {
            Domain::DomRange(r) => r.record_limit(level),
            Domain::DomValues(v) => v.record_limit(level),
        }
    }
    fn reduce_to_idx(&mut self, ele: usize, level: usize) -> usize {
        match self {
            Domain::DomRange(r) => r.reduce_to_idx(ele, level),
            Domain::DomValues(v) => v.reduce_to_idx(ele, level),
        }
    }

    fn reduce_to_value(&mut self, value: i32, level: usize) -> usize {
        match self {
            Domain::DomRange(r) => r.reduce_to_value(value, level),
            Domain::DomValues(v) => v.reduce_to_value(value, level),
        }
    }

    fn re_init(&mut self) {
        match self {
            Domain::DomRange(r) => r.re_init(),
            Domain::DomValues(v) => v.re_init(),
        }
    }

    fn values_at_position(&self, pos: usize) -> Option<i32> {
        match self {
            Domain::DomRange(r) => r.values_at_position(pos),
            Domain::DomValues(v) => v.values_at_position(pos),
        }
    }

    fn restore_limit(&mut self, level: usize) {
        match self {
            Domain::DomRange(r) => r.restore_limit(level),
            Domain::DomValues(v) => v.restore_limit(level),
        }
    }

    fn last_removed_level(&self) -> usize {
        match self {
            Domain::DomRange(r) => r.last_removed_level(),
            Domain::DomValues(v) => v.last_removed_level(),
        }
    }

    fn max_size(&self) -> usize {
        match self {
            Domain::DomRange(r) => r.max_size(),
            Domain::DomValues(v) => v.max_size(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Domain::DomRange(r) => r.size(),
            Domain::DomValues(v) => v.size(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Domain::DomRange(r) => r.is_empty(),
            Domain::DomValues(v) => v.is_empty(),
        }
    }

    fn next_idx(&self, current_idx: usize) -> Option<&usize> {
        match self {
            Domain::DomRange(r) => r.next_idx(current_idx),
            Domain::DomValues(v) => v.next_idx(current_idx),
        }
    }

    fn first_idx(&self) -> usize {
        match self {
            Domain::DomRange(r) => r.first_idx(),
            Domain::DomValues(v) => v.first_idx(),
        }
    }

    fn last_idx(&self) -> usize {
        match self {
            Domain::DomRange(r) => r.last_idx(),
            Domain::DomValues(v) => v.last_idx(),
        }
    }

    fn minimum(&self) -> i32 {
        match self {
            Domain::DomRange(r) => r.minimum(),
            Domain::DomValues(v) => v.minimum(),
        }
    }

    fn maximum(&self) -> i32 {
        match self {
            Domain::DomRange(r) => r.maximum(),
            Domain::DomValues(v) => v.maximum(),
        }
    }

    fn is_boolean(&self) -> bool {
        match self {
            Domain::DomRange(r) => r.is_boolean(),
            Domain::DomValues(v) => v.is_boolean(),
        }
    }

    fn is_single_value(&self, value: i32) -> bool {
        match self {
            Domain::DomRange(r) => r.is_single_value(value),
            Domain::DomValues(v) => v.is_single_value(value),
        }
    }

    fn contain_value(&self, value: i32) -> bool {
        match self {
            Domain::DomRange(r) => r.contain_value(value),
            Domain::DomValues(v) => v.contain_value(value),
        }
    }

    fn iter(&self) -> LinkedSetIter {
        match self {
            Domain::DomRange(r) => r.iter(),
            Domain::DomValues(v) => v.iter(),
        }
    }
}
