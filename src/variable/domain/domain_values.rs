/*
 * <p>@project_name: constraint_solver
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

pub struct DomainValues {
    elements: LinkedSet,
    n_assignment: Vec<usize>,
    values: Vec<usize>,
    map: HashMap<i32, usize>,
}

impl DomainValues {
    pub fn new(size: usize) -> Self {
        Self {
            elements: LinkedSet::new_with_fill(size),
            n_assignment: vec![],
            values: vec![],
            map: Default::default(),
        }
    }
}

impl Display for DomainValues {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.map)
    }
}

impl PartialEq for DomainValues {
    fn eq(&self, other: &Self) -> bool {
        if self.max_size() != other.max_size() {
            return false;
        }
        for i in 0..self.max_size() {
            if other.contain_value(self.idx_to_value(self.get_elements()[i])) {
                return false;
            }
        }
        return true;
    }
}

impl DomainTrait for DomainValues {
    #[inline]
    fn value_to_idx(&self, value: i32) -> usize {
        todo!()
    }
    #[inline]
    fn idx_to_value(&self, idx: usize) -> i32 {
        todo!()
    }
    #[inline]
    fn is_idx_correspond_to_values(&self) -> bool {
        todo!()
    }
    #[inline]
    fn hash(&self) -> usize {
        // let range:Range<usize> = 0..10;
        0
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
