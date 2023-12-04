/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/21 20:00
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::utils::linked_set::LinkedSet;
use crate::variable::domain::domain_trait::DomainTrait;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Index;

pub struct DomainValues {
    elements: LinkedSet,
    n_assignment: Vec<usize>,
    values: Vec<i32>,
    map: HashMap<i32, usize>,
}

impl DomainValues {
    pub fn new(vals: Vec<i32>) -> Self {
        let mut n_assignment = vec![];
        n_assignment.resize(vals.len(), 0);
        let mut values = vec![];
        let mut map: HashMap<i32, usize> = Default::default();
        for i in 0..vals.len() {
            values.push(vals[i]);
            map.insert(vals[i], i);
        }
        Self {
            elements: LinkedSet::new_with_fill(vals.len()),
            n_assignment,
            values,
            map,
        }
    }
}

impl Display for DomainValues {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl PartialEq for DomainValues {
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
        return true;
    }
}

impl Clone for DomainValues {
    fn clone(&self) -> Self {
        Self {
            elements: self.elements.clone(),
            n_assignment: self.n_assignment.clone(),
            values: self.values.clone(),
            map: self.map.clone(),
        }
    }
}

impl Index<usize> for DomainValues {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.get_elements()[index]
    }
}
impl DomainTrait for DomainValues {
    /// if the value is not in the domain, return usize::None
    #[inline]
    fn value_to_idx(&self, value: i32) -> Option<usize> {
        return if self.map.contains_key(&value) {
            match self.map.get(&value) {
                None => None,
                Some(e) => Some(*e),
            }
        } else {
            None
        };
    }

    /// if the value is not in the domain, return usize::None
    #[inline]
    fn idx_to_value(&self, idx: usize) -> Option<i32> {
        debug_assert!(idx < self.values.len());
        Some(self.values[idx])
    }
    #[inline]
    fn is_idx_correspond_to_values(&self) -> bool {
        self.minimum_value() == 0 && self.maximum_value() == self.max_size() as i32
    }
    #[inline]
    fn hash(&self) -> usize {
        // let range:Range<usize> = 0..10;

        let mut ret = 0;
        for i in self.values.iter().enumerate() {
            ret += *i.1 as usize * 31usize + (self.values.len() - i.0);
        }
        ret
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
