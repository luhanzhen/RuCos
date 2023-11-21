use crate::utils::linked_set::LinkedSet;
use std::fmt::Display;
use std::hash::Hash;

/**
 * @project_name: constraint_solver
 * @author: luhan zhen
 * @date:  2023/11/2 13:21
 *
 * @email: zhenlh20@mails.jlu.edu.cn
 *
 * @version: 1.0
 *
 * @description:
 *
 */

pub trait DomainTrait: Display {
    fn value_to_idx(&self, value: i32) -> usize;

    fn idx_to_value(&self, idx: usize) -> i32;

    fn is_idx_correspond_to_values(&self) -> bool;

    fn hash(&self) -> usize;

    fn get_elements(&self) -> &LinkedSet;
}
