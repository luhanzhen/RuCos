use crate::utils::linked_set::LinkedSet;
use crate::utils::set_trait::SetTrait;
use std::fmt::Display;

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

    fn get_elements(&mut self) -> &mut LinkedSet;
    #[inline]
    fn delete_idx(&mut self, idx: usize, level: usize) {
        self.get_elements().delete_at_level(idx, level)
    }
    #[inline]
    fn delete_value(&mut self, value: i32, level: usize) {
        let idx = self.value_to_idx(value);
        self.get_elements().delete_at_level(idx, level)
    }
    #[inline]
    fn reduce_to(&mut self, ele: usize, level: usize) -> usize {
        self.get_elements().reduce_to(ele, level)
    }

    #[inline]
    fn re_init(&mut self) {
        self.get_elements().fill()
    }
}
