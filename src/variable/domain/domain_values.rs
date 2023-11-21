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
use std::hash::{Hash, Hasher};

pub struct DomainValues {
    elements: LinkedSet,
    n_assignment: Box<Vec<usize>>,
    values: Box<Vec<usize>>,
    map: HashMap<i32, usize>,
}

impl DomainValues {
    fn new(size: usize) -> Self {
        Self {
            elements: LinkedSet::new_with_fill(size),
            n_assignment: Box::new(vec![]),
            values: Box::new(vec![]),
            map: Default::default(),
        }
    }
}

impl Display for DomainValues {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {}
}

impl DomainTrait for DomainValues {
    fn value_to_idx(&self, value: i32) -> usize {
        todo!()
    }

    fn idx_to_value(&self, idx: usize) -> i32 {
        todo!()
    }

    fn is_idx_correspond_to_values(&self) -> bool {
        todo!()
    }

    fn get_elements(&self) -> &LinkedSet {
        &self.elements
    }
}
