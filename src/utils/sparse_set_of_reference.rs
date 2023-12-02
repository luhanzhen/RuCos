/*
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/16 12:59
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::utils::set_trait::SetTrait;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use std::rc::Rc;

struct SparseSetOfReference<T>
// where T: Clone
{
    positions: Vec<usize>,
    elements: Vec<Rc<RefCell<T>>>,
    limit: usize,
    max_size: usize,
}

impl<T> Display for SparseSetOfReference<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("elements[limits]: ");
        // for i in self.iter() {
        //     str.push_str(&*i.to_string());
        //     // str.push_str("[");
        //     // str.push_str(&*self.limits(i).to_string());
        //     str.push_str(", ");
        // }
        str.push_str("\n");
        write!(f, "{}", str)
    }
}

impl<T> Index<usize> for SparseSetOfReference<T> {
    type Output = Rc<RefCell<T>>;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.max_size());
        &self.elements[index]
    }
}

impl<T> IndexMut<usize> for SparseSetOfReference<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.max_size());
        &mut self.elements[index]
    }
}

#[allow(dead_code)]
impl<T> SparseSetOfReference<T> {
    fn new(size: usize, vec_t: &Vec<Rc<RefCell<T>>>, fill: bool) -> Self {
        let mut elements = vec![];
        let mut positions = vec![];

        positions.resize(size, 0usize);
        for i in 0..size {
            elements.push(Rc::clone(&vec_t[i]));
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
}

impl<T> Clone for SparseSetOfReference<T> {
    fn clone(&self) -> Self {
        Self {
            positions: self.positions.clone(),
            elements: self.elements.clone(),
            limit: self.limit,
            max_size: self.max_size,
        }
    }
}

impl<T> SetTrait<T> for SparseSetOfReference<T> {
    fn add(&mut self, _ele: T) {
        todo!()
    }

    fn fill(&mut self) {
        todo!()
    }

    fn delete(&mut self, _ele: T) {
        // debug_assert!(ele < self.max_size());
        // if !self.contains(ele) {
        //     return;
        // }
        // self.limit -= 1;
        // let tmp = self.elements[self.limit];
        // let pos = self.positions[ele];
        // self.positions[tmp] = pos;
        // self.elements[pos] = tmp;
        //
        // self.elements[self.limit] = ele;
        // self.positions[ele] = self.limit;
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

pub struct SparseSetOfReferenceIter<'a, T> {
    index: usize,
    value: &'a SparseSetOfReference<T>,
}

impl<T> Iterator for SparseSetOfReferenceIter<'_, T> {
    type Item = Rc<RefCell<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.index < self.value.limit {
            self.index += 1;
            Some(Rc::clone(&self.value[self.index - 1]))
        } else {
            None
        };
    }
}
