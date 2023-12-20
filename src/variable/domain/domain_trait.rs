use crate::utils::linked_set::{LinkedSet, LinkedSetIter};
use crate::utils::set_trait::SetTrait;
use rand::random;
use std::fmt::Display;
use std::ops::Index;

/***
 * @project_name: RuCos
 * @author: luhan zhen
 * @date:  2023/11/2 13:21
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
#[allow(dead_code)]
pub trait DomainTrait: Display + Clone + PartialEq + Index<usize> {
    /// if the value is not in the domain, return None
    fn value_to_idx(&self, value: i32) -> Option<usize>;

    /// if the value is not in the domain, return None
    fn idx_to_value(&self, idx: usize) -> Option<i32>;

    fn is_idx_correspond_to_values(&self) -> bool;

    fn hash(&self) -> usize;

    fn get_elements(&self) -> &LinkedSet;
    fn get_elements_mut(&mut self) -> &mut LinkedSet;

    #[inline]
    fn is_limit_recorded_at_level(&self, level: usize) -> bool {
        self.get_elements().is_limit_recorded_at_level(level)
    }

    fn update_idx_upper_bound_at_level(&mut self, update_idx: usize, level: usize) -> usize {
        self.get_elements_mut()
            .update_idx_upper_bound_at_level(update_idx, level)
    }

    fn update_idx_lower_bound_at_level(&mut self, update_idx: usize, level: usize) -> usize {
        self.get_elements_mut()
            .update_idx_lower_bound_at_level(update_idx, level)
    }
    #[inline]
    fn delete_idx_at_level(&mut self, idx: usize, level: usize) {
        self.get_elements_mut().delete_at_level(idx, level)
    }
    #[inline]
    fn delete_value_at_level(&mut self, value: i32, level: usize) {
        let idx = self.value_to_idx(value).unwrap();
        self.get_elements_mut().delete_at_level(idx, level);
    }

    #[inline]
    fn which_level_deleted_value(&self, value: i32) -> Option<usize> {
        match self.value_to_idx(value) {
            None => None,
            Some(idx) => self.get_elements().which_level_deleted(idx),
        }
    }

    #[inline]
    fn which_level_deleted_idx(&self, idx: usize) -> Option<usize> {
        self.get_elements().which_level_deleted(idx)
    }

    #[inline]
    fn record_limit(&mut self, level: usize) {
        self.get_elements_mut().record_limit(level)
    }
    #[inline]
    fn reduce_to_idx(&mut self, ele: usize, level: usize) -> usize {
        self.get_elements_mut().reduce_to(ele, level)
    }

    #[inline]
    fn reduce_to_value(&mut self, value: i32, level: usize) -> usize {
        match self.value_to_idx(value) {
            None => self.size(),
            Some(v) => self.get_elements_mut().reduce_to(v, level),
        }
    }

    #[inline]
    fn re_init(&mut self) {
        self.get_elements_mut().fill()
    }
    #[inline]
    fn values_at_position(&self, pos: usize) -> Option<i32> {
        let val = self.get_elements()[pos];
        self.idx_to_value(val)
    }
    #[inline]
    fn restore_to_limit(&mut self, level: usize) {
        self.get_elements_mut().restore_limit(level);
    }
    #[inline]
    fn last_removed_level(&self) -> usize {
        self.get_elements().last_removed_level()
    }
    #[inline]
    fn max_size(&self) -> usize {
        self.get_elements().max_size()
    }
    #[inline]
    fn size(&self) -> usize {
        self.get_elements().size()
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.get_elements().is_empty()
    }
    #[inline]
    fn next_idx(&self, current_idx: usize) -> Option<&usize> {
        self.get_elements().next(current_idx)
    }
    #[inline]
    fn first_idx(&self) -> usize {
        self.get_elements().first()
    }
    #[inline]
    fn last_idx(&self) -> usize {
        self.get_elements().last()
    }
    #[inline]
    fn minimum_value(&self) -> i32 {
        debug_assert!(self.size() > 0);
        let mut min = i32::MAX;
        for ele in self.iter() {
            let v = match self.idx_to_value(ele) {
                None => i32::MAX,
                Some(e) => e,
            };
            if min > v {
                min = v;
            }
        }
        min
    }
    #[inline]
    fn random_idx(&self) -> usize {
        let n = random::<usize>() & self.max_size();
        self.get_elements()[n]
    }

    #[inline]
    fn minimum_idx(&self) -> usize {
        debug_assert!(self.size() > 0);
        self.first_idx()
    }

    #[inline]
    fn maximum_idx(&self) -> usize {
        debug_assert!(self.size() > 0);
        self.last_idx()
    }

    #[inline]
    fn maximum_value(&self) -> i32 {
        debug_assert!(self.size() > 0);
        let mut max = i32::MIN;
        for ele in self.iter() {
            let v = match self.idx_to_value(ele) {
                None => i32::MIN,
                Some(e) => e,
            };
            if max < v {
                max = v;
            }
        }
        max
    }
    #[inline]
    fn is_boolean(&self) -> bool {
        self.max_size() == 2 && self.maximum_value() == 1 && self.minimum_value() == 0
    }

    #[inline]
    fn is_single_value(&self, value: i32) -> bool {
        self.size() == 1 && self.contains_value(value)
    }
    #[inline]
    fn contains_value(&self, value: i32) -> bool {
        match self.value_to_idx(value) {
            None => false,
            Some(r) => self.get_elements().contains(r),
        }
    }
    #[inline]
    fn contains_idx(&self, idx: usize) -> bool {
        self.get_elements().contains(idx)
    }

    fn iter(&self) -> LinkedSetIter {
        self.get_elements().iter()
    }

    fn str(&self) -> String {
        let mut str = String::from("elements[ ");
        for e in self.get_elements().iter() {
            str.push_str(&self.idx_to_value(e).unwrap().to_string());
            str.push_str(", ");
        }
        str.push_str("] deleted[");
        for i in 0..self.max_size() {
            if !self.get_elements().contains(i) {
                str.push_str(&self.idx_to_value(i).unwrap().to_string());
                str.push_str("(");
                str.push_str(
                    &self
                        .get_elements()
                        .which_level_deleted(i)
                        .unwrap()
                        .to_string(),
                );
                str.push_str("), ");
            }
        }
        str.push_str("]");
        str.push_str("\n");
        str
    }
}
