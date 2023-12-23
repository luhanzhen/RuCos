/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/21 20:00
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::utils::linked_set::LinkedSet;
use crate::utils::set_trait::SetTrait;
use crate::variable::domain::domain_trait::DomainTrait;
use std::fmt::{Display, Formatter};
use std::ops::{Index, Range};
#[derive(Debug)]
pub struct DomainRange {
    elements: LinkedSet,
    n_assignment: Vec<usize>,
    range: Range<i32>,
}

impl DomainRange {
    pub fn new(range: Range<i32>) -> Self {
        let mut n_assignment = vec![];
        n_assignment.resize((range.end - range.start + 1) as usize, 0);
        Self {
            elements: LinkedSet::new_with_fill(range.len()),
            n_assignment,
            range,
        }
    }
}

impl Display for DomainRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl PartialEq for DomainRange {
    fn eq(&self, other: &Self) -> bool {
        if self.max_size() != other.max_size() {
            return false;
        }
        for i in 0..self.max_size() {
            let e = self.idx_to_value(self.get_elements()[i]).unwrap();
            if other.contains_value(e) {
                return false;
            }
        }
        true
    }
}

impl Clone for DomainRange {
    fn clone(&self) -> Self {
        Self {
            elements: self.get_elements().clone(),
            n_assignment: self.n_assignment.clone(),
            range: self.range.clone(),
        }
    }
}

impl Index<usize> for DomainRange {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.get_elements()[index]
    }
}

impl DomainTrait for DomainRange {
    fn value_to_idx(&self, value: i32) -> Option<usize> {
        if value < self.range.start || value > self.range.end {
            None
        } else {
            Some((value - self.range.start) as usize)
        }
    }

    fn idx_to_value(&self, idx: usize) -> Option<i32> {
        if idx >= self.get_elements().max_size() {
            None
        } else {
            Some(self.range.start + idx as i32)
        }
    }

    fn is_idx_correspond_to_values(&self) -> bool {
        false
    }

    fn hash(&self) -> usize {
        let mut ret = 0;
        ret += (self.range.start * 31) ^ 1;
        ret += (self.range.end * 31) ^ 2;
        ret as usize
    }

    #[inline]
    fn get_elements(&self) -> &LinkedSet {
        &self.elements
    }

    #[inline]
    fn get_elements_mut(&mut self) -> &mut LinkedSet {
        &mut self.elements
    }
}
