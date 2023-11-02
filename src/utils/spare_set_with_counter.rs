/*
 * <p>@project_name: CConstraintSolver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/2 12:42
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::utils::spare_set::{SpareSet, SpareSetIter};
use crate::utils::trait_set::Set;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

struct SpareSetCounter {
    set: SpareSet,
    _counter: Box<Vec<usize>>,
}

impl Display for SpareSetCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("elements[counter]: ");
        for i in self.iter() {
            str.push_str(&*i.to_string());
            str.push_str("[");
            str.push_str(&*self.counter(i).to_string());
            str.push_str("], ");
        }
        str.push_str("\n");
        write!(f, "{}", str)
    }
}

impl Clone for SpareSetCounter {
    fn clone(&self) -> Self {
        let mut _counter = Box::new(vec![]);
        _counter.copy_from_slice(&**self._counter);
        Self {
            set: self.set.clone(),
            _counter,
        }
    }
}

impl Index<usize> for SpareSetCounter {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.max_size());
        &self.set[index]
    }
}

impl IndexMut<usize> for SpareSetCounter {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.max_size());
        &mut self.set[index]
    }
}

#[allow(dead_code)]
impl SpareSetCounter {
    pub fn new(size: usize) -> Self {
        let mut _counter = Box::new(vec![]);
        _counter.resize(size, 0usize);
        Self {
            set: SpareSet::new(size),
            _counter,
        }
    }
    pub fn new_with_fill(size: usize) -> Self {
        let mut _counter = Box::new(vec![]);
        _counter.resize(size, 0usize);
        Self {
            set: SpareSet::new_with_fill(size),
            _counter,
        }
    }

    pub fn iter(&self) -> SpareSetIter {
        self.set.iter()
    }

    pub fn counter(&self, ele: usize) -> usize {
        return self._counter[ele];
    }
}

impl Set for SpareSetCounter {
    fn add(&mut self, ele: usize) {
        if self.contains(ele) {
            self._counter[ele] += 1
        } else {
            self.set.add(ele);
            self._counter[ele] = 1
        }
    }

    fn delete(&mut self, ele: usize) {
        self.set.delete(ele)
    }

    fn contains(&self, ele: usize) -> bool {
        self.set.contains(ele)
    }

    fn size(&self) -> usize {
        self.set.size()
    }

    fn clear(&mut self) {
        self.set.clear()
    }

    fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    fn max_size(&self) -> usize {
        self.set.max_size()
    }
}
