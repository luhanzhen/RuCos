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
    fn new(size: usize, fill: bool) -> Self {
        let mut elements = vec![];
        let mut positions = vec![];
        // elements.resize(size, 0usize);

        positions.resize(size, 0usize);
        for i in 0..size {
            // elements[i] = i;
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
        todo!()
    }
}

impl<T> SetTrait<T> for SparseSetOfReference<T> {
    fn add(&mut self, ele: T) {
        todo!()
    }

    fn delete(&mut self, ele: T) {
        todo!()
    }

    fn contains(&self, ele: usize) -> bool {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn max_size(&self) -> usize {
        todo!()
    }
}
