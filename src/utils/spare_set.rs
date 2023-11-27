/*
* <p>@project_name: constraint_solver
* </p>
* <p>@author: luhan zhen
* </p>
* <p>@date:  2023/11/1 22:00
* </p>
* <p>@email: 940864649@qq.com
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

use crate::utils::set_trait::SetTrait;
use std::fmt::{Display, Formatter};
use std::ops::Index;

pub struct SpareSet {
    positions: Vec<usize>,
    elements: Vec<usize>,
    limit: usize,
    max_size: usize,
}

impl Display for SpareSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("elements: ");
        for i in self.iter() {
            str.push_str(&*i.to_string());
            str.push_str(", ")
        }
        str.push_str("\n");
        write!(f, "{}", str)
    }
}

impl Clone for SpareSet {
    fn clone(&self) -> Self {
        Self {
            positions: self.positions.clone(),
            elements: self.elements.clone(),
            limit: self.limit,
            max_size: self.max_size,
        }
    }
}

pub struct SpareSetIter<'a> {
    index: usize,
    value: &'a SpareSet,
}

impl Iterator for SpareSetIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.index < self.value.limit {
            self.index += 1;
            Some(self.value[self.index - 1])
        } else {
            None
        };
    }
}

impl Index<usize> for SpareSet {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.max_size());
        &self.elements[index]
    }
}

// impl IndexMut<usize> for SpareSet {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         debug_assert!(index < self.max_size());
//         &mut self.elements[index]
//     }
// }

#[allow(dead_code)]
impl SpareSet {
    pub(crate) fn limit(&mut self) -> &mut usize {
        &mut self.limit
    }
    pub fn iter(&self) -> SpareSetIter {
        SpareSetIter {
            index: 0,
            value: &self,
        }
    }
    pub fn new_without_fill(size: usize) -> Self {
        SpareSet::new(size, false)
    }

    pub fn new_with_fill(size: usize) -> Self {
        SpareSet::new(size, true)
    }

    fn new(size: usize, fill: bool) -> Self {
        let mut elements = vec![];
        let mut positions = vec![];
        elements.resize(size, 0usize);
        positions.resize(size, 0usize);
        for i in 0..size {
            elements[i] = i;
            positions[i] = i;
        }
        Self {
            positions,
            elements,
            limit: match fill {
                true => size,
                false => 0,
            },
            max_size: size,
        }
    }

    pub fn get_position(&self, ele: usize) -> usize {
        debug_assert!(ele < self.max_size());
        self.positions[ele]
    }

    pub fn fill(&mut self) {
        self.limit = self.max_size
    }

    pub fn reduce_to(&mut self, ele: usize) {
        self.clear();
        self.add(ele);
    }
}

impl SetTrait<usize> for SpareSet {
    fn add(&mut self, ele: usize) {
        debug_assert!(ele < self.max_size());
        if self.contains(ele) {
            return;
        }
        let tmp = self.elements[self.limit];
        let pos = self.positions[ele];

        self.positions[tmp] = pos;
        self.elements[pos] = tmp;

        self.elements[self.limit] = ele;
        self.positions[ele] = self.limit;
        self.limit += 1;
    }

    fn fill(&mut self) {
        self.limit = self.max_size;
    }

    fn delete(&mut self, ele: usize) {
        debug_assert!(ele < self.max_size());
        if !self.contains(ele) {
            return;
        }
        self.limit -= 1;
        let tmp = self.elements[self.limit];
        let pos = self.positions[ele];
        self.positions[tmp] = pos;
        self.elements[pos] = tmp;

        self.elements[self.limit] = ele;
        self.positions[ele] = self.limit;
    }

    fn contains(&self, ele: usize) -> bool {
        debug_assert!(ele <= self.max_size());
        self.positions[ele] < self.limit
    }

    fn size(&self) -> usize {
        self.limit
    }
    fn clear(&mut self) {
        self.limit = 0
    }
    fn is_empty(&self) -> bool {
        self.limit == 0
    }
    fn max_size(&self) -> usize {
        self.max_size
    }
}
