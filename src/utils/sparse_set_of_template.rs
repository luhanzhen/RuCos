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

use crate::utils::trait_set::SetTrait;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

struct SparseSetOfTemplate<'a, T>
// where T: Clone
{
    positions: Box<Vec<usize>>,
    elements: Box<Vec<&'a T>>,
    limit: usize,
    max_size: usize,
}
impl<'a, T> Display for SparseSetOfTemplate<'a, T> {
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
impl<'a, T> Index<usize> for SparseSetOfTemplate<'a, T> {
    type Output = &'a T;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.max_size());
        &self.elements[index]
    }
}

impl<'a, T> IndexMut<usize> for SparseSetOfTemplate<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.max_size());
        &mut self.elements[index]
    }
}
#[allow(dead_code)]
impl<'a, T> SparseSetOfTemplate<'a, T> {
    fn new(size: usize, fill: bool) -> Self {
        let mut elements = Box::new(vec![]);
        let mut positions = Box::new(vec![]);
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
impl<'a, T> SetTrait<&'a T> for SparseSetOfTemplate<'a, T> {
    fn add(&mut self, ele: &'a T) {
        todo!()
    }

    fn delete(&mut self, ele: &'a T) {
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
