/*
* <p>@project_name: CConstraintSolver
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

use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

pub struct SpareSet {
    positions: Box<Vec<usize>>,
    elements: Box<Vec<usize>>,
    limit: usize,
    capacity: usize,
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
        let mut elements = Box::new(vec![]);
        let mut positions = Box::new(vec![]);
        elements.copy_from_slice(&**self.elements);
        positions.copy_from_slice(&**self.positions);
        Self {
            positions: elements,
            elements: positions,
            limit: self.limit,
            capacity: self.capacity,
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
        if self.index < self.value.limit {
            self.index += 1;
            return Some(self.value.elements[self.index - 1]);
        } else {
            return None;
        }
    }
}

impl Index<usize> for SpareSet {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.max_size());
        &self.elements[index]
    }
}

impl IndexMut<usize> for SpareSet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.max_size());
        &mut self.elements[index]
    }
}

impl SpareSet {
    pub fn iter(&self) -> SpareSetIter {
        SpareSetIter {
            index: 0,
            value: &self,
        }
    }
    pub fn new(size: usize) -> Self {
        SpareSet::get(size, false)
    }

    pub fn new_with_fill(size: usize) -> Self {
        SpareSet::get(size, true)
    }

    fn get(size: usize, fill: bool) -> Self {
        let mut elements = Box::new(vec![]);
        let mut positions = Box::new(vec![]);
        elements.resize(size, 0usize);
        positions.resize(size, 0usize);
        for i in 0..size {
            elements[i] = i as usize;
            positions[i] = i as usize;
        }

        Self {
            positions,
            elements,
            limit: match fill {
                true => size,
                false => 0,
            },
            capacity: size,
        }
    }

    pub fn add(&mut self, ele: usize) {
        assert!(ele < self.max_size() as usize);
        if self.contains(ele) {
            return;
        }
        let tmp = self.elements[self.limit];
        let pos = self.positions[ele as usize];

        self.positions[tmp] = pos;
        self.elements[pos] = tmp;

        self.elements[self.limit] = ele;
        self.positions[ele] = self.limit;
        self.limit += 1;
    }
    pub fn reduce_to(&mut self, ele: usize) {
        self.clear();
        self.add(ele);
    }
    pub fn get_position(&self, ele: usize) -> usize {
        assert!(ele < self.max_size() as usize);
        self.positions[ele]
    }
    pub fn del(&mut self, ele: usize) {
        assert!(ele < self.max_size() as usize);
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

    pub fn contains(&self, ele: usize) -> bool {
        assert!(ele <= self.max_size() as usize);
        self.positions[ele] < self.limit
    }

    pub fn max_size(&self) -> usize {
        self.capacity
    }
    pub fn size(&self) -> usize {
        self.limit
    }
    pub fn is_empty(&self) -> bool {
        self.limit == 0
    }
    pub fn clear(&mut self) {
        self.limit = 0
    }

    pub fn fill(&mut self) {
        self.limit = self.capacity
    }
}
