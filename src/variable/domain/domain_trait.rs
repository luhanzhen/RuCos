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
    #[inline]
    fn values_at_position(&mut self, pos: usize) -> i32 {
        let val = self.get_elements()[pos];
        self.idx_to_value(val)
    }
    #[inline]
    fn restore_limit(&mut self, level: usize) {
        self.get_elements().record_limit(level);
    }
    #[inline]
    fn last_removed_level(&mut self) -> usize {
        self.get_elements().last_removed_level()
    }
    #[inline]
    fn max_size(&mut self) -> usize {
        self.get_elements().max_size()
    }
    #[inline]
    fn size(&mut self) -> usize {
        self.get_elements().size()
    }
    #[inline]
    fn is_empty(&mut self) -> bool {
        self.get_elements().is_empty()
    }
    #[inline]
    fn next_idx(&mut self, current_idx: usize) -> Option<&usize> {
        self.get_elements().next(current_idx)
    }
    #[inline]
    fn first_idx(&mut self) -> usize {
        self.get_elements().first()
    }
    #[inline]
    fn last_idx(&mut self) -> usize {
        self.get_elements().last()
    }
    #[inline]
    fn minimum(&mut self) -> i32 {
        debug_assert!(self.size() > 0);
        let t = self.first_idx();
        self.idx_to_value(t)
    }

    #[inline]
    fn maximum(&mut self) -> i32 {
        debug_assert!(self.size() > 0);
        let t = self.last_idx();
        self.idx_to_value(t)
    }
    #[inline]
    fn is_boolean(&mut self) -> bool {
        self.max_size() == 2 && self.maximum() == 1 && self.minimum() == 0
    }
}
