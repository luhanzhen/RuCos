/*
 * <p>@project_name: CConstraintSolver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/2 13:28
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use crate::utils::trait_set::Set;
use std::fmt::{Display, Formatter};
use std::ops::Index;

const NOT_STORE: i32 = -1;

pub struct LinkedSet {
    size: usize,
    first: usize,
    last: usize,
    last_removed: usize,
    nb_levels: usize,
    limit: Box<Vec<usize>>,
    prev: Box<Vec<usize>>,
    removed_levels: Box<Vec<usize>>,
    prev_removed: Box<Vec<usize>>,
    next: Box<Vec<usize>>,
}

#[allow(dead_code)]
impl LinkedSet {
    pub fn reduce_to(&mut self, ele: usize, level: usize) -> usize {
        let last_size = self.size;

        let mut e = self.first;
        loop {
            match self.next(e) {
                None => break,
                Some(ee) => {
                    if e != ele {
                        self.delete_at_level(e, level)
                    }
                    e = ee;
                }
            }
        }

        last_size
    }

    pub fn next(&self, ele: usize) -> Option<usize> {
        debug_assert!(ele < self.max_size());
        if self.removed_levels[ele] == usize::MAX
        {
            return Some(self.next[ele]);
        }
        if ele < self.first
        {
            return Some(self.first);
        }
        let mut next = self.next[ele];
        if next == usize::MAX || next > self.last
        {
            return None;
        }
        while self.removed_levels[next] != usize::MAX {
            next = self.next[next];
        }

         Some(next)
    }
    pub fn prev(&self, ele: usize) -> Option<usize> {
        debug_assert!(ele < self.max_size());
        if self.removed_levels[ele] == usize::MAX
        {
            return Some(self.prev[ele]);
        }
        if ele > self.last
        {
            return Some(self.last);
        }


    }

    pub fn delete_at_level(&mut self, ele: usize, level: usize) {
        debug_assert!(ele < self.removed_levels.len());
        self.removed_levels[ele] = level;
        self.size -= 1;
        self.delete(ele);
    }

    pub fn iter(&self) -> LinkedSetIter {
        LinkedSetIter {
            index: 0,
            value: &self,
        }
    }
    pub fn new(size: usize) -> Self {
        LinkedSet::get(size, false)
    }
    pub fn new_with_fill(size: usize) -> Self {
        LinkedSet::get(size, true)
    }

    fn get(size: usize, fill: bool) -> Self {
        let mut limit = Box::new(vec![]);
        let mut prev = Box::new(Vec::with_capacity(size));
        let mut removed_levels = Box::new(Vec::with_capacity(size));
        let mut prev_removed = Box::new(Vec::with_capacity(size));
        let mut next = Box::new(Vec::with_capacity(size));

        next[0] = 1;
        prev[0] = usize::MAX;
        for i in 1..size {
            prev[i] = i - 1;
            next[i] = i + 1;
        }
        next[size - 1] = usize::MAX;
        prev_removed.fill(usize::MAX);
        removed_levels.fill(usize::MAX);
        Self {
            size,
            first: 0,
            last: size - 1,
            last_removed: usize::MAX,
            nb_levels: 0,
            limit,
            prev,
            removed_levels,
            prev_removed,
            next,
        }
    }
}

impl Display for LinkedSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Clone for LinkedSet {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            first: self.first,
            last: self.last,
            last_removed: self.last_removed,
            nb_levels: self.nb_levels,
            limit: self.limit.clone(),
            prev: self.prev.clone(),
            removed_levels: self.removed_levels.clone(),
            prev_removed: self.prev_removed.clone(),
            next: self.next.clone(),
        }
    }
}

impl Set for LinkedSet {
    fn add(&mut self, ele: usize) {
        debug_assert!(ele < self.max_size());
        let prev = self.prev[ele];
        let next = self.next[ele];
        if prev == usize::MAX {
            self.first = ele;
        } else {
            self.next[prev] = ele;
        }
        if next == usize::MAX {
            self.last = ele;
        } else {
            self.prev[next] = ele;
        }
        self.last_removed = self.prev_removed[ele];
    }

    fn delete(&mut self, ele: usize) {
        debug_assert!(ele < self.max_size());
        let prev = self.prev[ele];
        let next = self.next[ele];
        if prev == usize::MAX {
            self.first = next;
        } else {
            self.next[prev] = next;
        }
        if next == usize::MAX {
            self.last = prev;
        } else {
            self.prev[next] = prev;
        }
        self.prev_removed[ele] = self.last_removed;
        self.last_removed = ele;
    }

    fn contains(&self, ele: usize) -> bool {
        debug_assert!(ele < self.max_size());
        self.removed_levels[ele] == usize::MAX
    }

    fn size(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn max_size(&self) -> usize {
        self.next.len()
    }
}

impl Index<usize> for LinkedSet {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.max_size());
        let mut e = self.first;
        for _ in 0..index {
            match self.next(e) {
                None => {
                    panic!("wrong index for LinkedSet!!!")
                }
                Some(ee) => {
                    e = ee;
                }
            }
        }
        return &e;
    }
}

pub struct LinkedSetIter<'a> {
    index: usize,
    value: &'a LinkedSet,
}

impl Iterator for LinkedSetIter<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
