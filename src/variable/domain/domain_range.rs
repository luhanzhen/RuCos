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
use std::ops::Range;

pub struct DomainRange {
    elements: LinkedSet,
    n_assignment: Vec<usize>,
    values: Vec<usize>,
    map: HashMap<i32, usize>,
}

impl DomainRange {
    pub fn new(a: Range<usize>) -> Self {
        Self {
            elements: LinkedSet::new_with_fill(a.len()),
            n_assignment: vec![],
            values: vec![],
            map: Default::default(),
        }
    }
}

impl Display for DomainRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PartialEq for DomainRange {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl DomainTrait for DomainRange {
    fn value_to_idx(&self, value: i32) -> usize {
        todo!()
    }

    fn idx_to_value(&self, idx: usize) -> i32 {
        todo!()
    }

    fn is_idx_correspond_to_values(&self) -> bool {
        todo!()
    }

    fn hash(&self) -> usize {
        todo!()
    }

    fn get_elements(&self) -> &LinkedSet {
        todo!()
    }

    fn get_elements_mut(&mut self) -> &mut LinkedSet {
        todo!()
    }
}
