/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/2 13:28
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::utils::set_trait::SetTrait;
use std::fmt::{Display, Formatter};
use std::ops::Index;
#[derive(Debug)]
pub struct LinkedSet {
    size: usize,
    first: usize,
    last: usize,
    last_removed: usize,
    nb_levels: usize,
    limits: Vec<usize>,
    prev: Vec<usize>,
    removed_levels: Vec<usize>,
    prev_removed: Vec<usize>,
    next: Vec<usize>,
}

const INVALID: usize = usize::MAX;

#[allow(dead_code)]
impl LinkedSet {
    pub fn reduce_to(&mut self, ele: usize, level: usize) -> usize {
        let last_size = self.size;
        // for i in 0..self.max_size() {
        //     if i != ele {
        //         self.delete_at_level(i, level)
        //     }
        // }
        let mut begin = self.first();
        loop {
            match self.next(begin) {
                None => break,
                Some(&next) => {
                    if begin != ele {
                        self.delete_at_level(begin, level);
                    }
                    begin = next;
                }
            }
        }
        last_size
    }

    pub fn update_idx_upper_bound_at_level(&mut self, update_idx: usize, level: usize) -> usize {
        let last_size = self.size;
        if update_idx <= self.first() {
            return last_size;
        }
        let mut begin = update_idx;
        loop {
            match self.next(begin) {
                None => break,
                Some(&next) => {
                    self.delete_at_level(begin, level);
                    if next < update_idx {
                        break;
                    }
                    begin = next;
                }
            }
        }
        last_size
    }

    pub fn update_idx_lower_bound_at_level(&mut self, update_idx: usize, level: usize) -> usize {
        let last_size = self.size;
        if update_idx >= self.last() {
            return last_size;
        }
        let mut begin = self.first();
        loop {
            match self.next(begin) {
                None => break,
                Some(&next) => {
                    self.delete_at_level(begin, level);
                    if update_idx <= next {
                        break;
                    }
                    begin = next;
                }
            }
        }
        last_size
    }

    pub fn next(&self, ele: usize) -> Option<&usize> {
        if ele >= self.max_size() {
            return None;
        }

        if self.removed_levels[ele] == INVALID {
            return Some(&self.next[ele]);
        }
        if ele < self.first {
            return Some(&self.first);
        }
        let mut next = &self.next[ele];
        if *next == INVALID || *next > self.last {
            return Some(&INVALID);
        }
        while self.removed_levels[*next] != INVALID {
            next = &self.next[*next];
        }

        Some(next)
    }

    pub fn first(&self) -> usize {
        self.first
    }

    pub fn last(&self) -> usize {
        self.last
    }

    pub fn record_limit(&mut self, level: usize) {
        if level >= self.limits.len() {
            self.limits.resize(level + 1, INVALID);
        }
        if self.limits[level] == INVALID {
            self.limits[level] = self.size
        }
    }

    pub fn restore_last_dropped(&mut self) {
        debug_assert!(self.last_removed != INVALID && !self.contains(self.last_removed));
        self.removed_levels[self.last_removed] = INVALID;
        self.size += 1;
        self.add(self.last_removed);
    }

    pub fn restore_limit(&mut self, level: usize) {
        debug_assert!(self.last_removed != INVALID);
        debug_assert!(self.removed_levels[self.last_removed] <= level);
        let mut t = self.last_removed;
        while t != INVALID && self.removed_levels[t] >= level {
            self.restore_last_dropped();
            t = self.last_removed;
        }
        self.limits[level] = INVALID;
    }

    pub fn is_limit_recorded_at_level(&self, level: usize) -> bool {
        level < self.limits.len() && self.limits[level] != INVALID
    }
    pub fn last_removed_level(&self) -> usize {
        match self.last_removed == INVALID {
            true => INVALID,
            false => self.removed_levels[self.last_removed],
        }
    }
    pub fn prev(&self, ele: usize) -> Option<usize> {
        debug_assert!(ele < self.max_size());
        if self.removed_levels[ele] == INVALID {
            return Some(self.prev[ele]);
        }
        if ele > self.last {
            return Some(self.last);
        }
        let mut prev = self.prev[ele];
        if prev < self.first {
            return Some(INVALID);
        }
        while self.removed_levels[prev] != INVALID {
            prev = self.prev[prev];
        }
        Some(prev)
    }

    pub fn delete_at_level(&mut self, ele: usize, level: usize) {
        debug_assert!(ele < self.removed_levels.len());
        if !self.contains(ele) {
            return;
        }
        self.removed_levels[ele] = level;
        self.size -= 1;
        self.delete(ele);
    }

    pub fn which_level_deleted(&self, ele: usize) -> Option<usize> {
        debug_assert!(ele < self.removed_levels.len());
        if self.removed_levels[ele] == INVALID {
            None
        } else {
            Some(self.removed_levels[ele])
        }
    }

    pub fn iter(&self) -> LinkedSetIter {
        LinkedSetIter {
            index: self.first,
            value: self,
        }
    } // pub fn new(size: usize) -> Self {
      //     LinkedSet::get(size, )
      // }

    // pub fn new_with_fill(size: usize) -> Self {
    //     LinkedSet::get(size, true)
    // }

    pub fn new_with_fill(size: usize) -> Self {
        let mut limit = vec![];
        limit.reserve(size);
        let mut prev = Vec::with_capacity(size);
        let mut removed_levels = Vec::with_capacity(size);
        let mut prev_removed = Vec::with_capacity(size);
        let mut next = Vec::with_capacity(size);
        prev.resize(size, INVALID);
        removed_levels.resize(size, INVALID);
        prev_removed.resize(size, INVALID);
        next.resize(size, INVALID);
        next[0] = 1;

        for i in 1..size {
            prev[i] = i - 1;
            next[i] = i + 1;
        }

        next[size - 1] = INVALID;
        prev_removed.fill(INVALID);
        removed_levels.fill(INVALID);
        Self {
            size,
            first: 0,
            last: size - 1,
            last_removed: INVALID,
            nb_levels: 0,
            limits: limit,
            prev,
            removed_levels,
            prev_removed,
            next,
        }
    }
}

impl Display for LinkedSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("elements[ ");
        for e in self.iter() {
            str.push_str(&e.to_string());
            str.push_str(", ");
        }
        str.push_str("] deleted[");
        for i in 0..self.max_size() {
            if !self.contains(i) {
                str.push_str(&i.to_string());
                str.push_str(", ");
            }
        }
        str.push(']');
        str.push('\n');
        write!(f, "{}", str)
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
            limits: self.limits.clone(),
            prev: self.prev.clone(),
            removed_levels: self.removed_levels.clone(),
            prev_removed: self.prev_removed.clone(),
            next: self.next.clone(),
        }
    }
}

impl SetTrait<usize> for LinkedSet {
    fn add(&mut self, ele: usize) {
        debug_assert!(ele == self.last_removed);
        let prev = self.prev[ele];
        let next = self.next[ele];
        if prev == INVALID {
            self.first = ele;
        } else {
            self.next[prev] = ele;
        }
        if next == INVALID {
            self.last = ele;
        } else {
            self.prev[next] = ele;
        }
        self.last_removed = self.prev_removed[ele];
    }

    fn fill(&mut self) {
        self.first = 0;
        self.last = self.max_size() - 1;
        for i in 1..self.prev.len() {
            self.prev[i] = i - 1;
        }
        for i in 0..self.next.len() {
            self.next[i] = i + 1;
        }
        let n = self.next.len() - 1;
        self.prev[0] = INVALID;
        self.next[n] = INVALID;
        self.last_removed = INVALID;
        self.prev_removed.fill(INVALID);
        self.removed_levels.fill(INVALID);
    }

    fn delete(&mut self, ele: usize) {
        debug_assert!(ele < self.max_size());
        let prev = self.prev[ele];
        let next = self.next[ele];
        if prev == INVALID {
            self.first = next;
        } else {
            self.next[prev] = next;
        }
        if next == INVALID {
            self.last = prev;
        } else {
            self.prev[next] = prev;
        }
        self.prev_removed[ele] = self.last_removed;
        self.last_removed = ele;
    }
    #[inline]
    fn contains(&self, ele: usize) -> bool {
        debug_assert!(ele < self.max_size());
        self.removed_levels[ele] == INVALID
    }
    #[inline]
    fn size(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        let mut begin = self.first();
        loop {
            match self.next(begin) {
                None => break,
                Some(&next) => {
                    self.delete(begin);
                    begin = next;
                }
            }
        }
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    #[inline]
    fn max_size(&self) -> usize {
        self.next.len()
    }
}

impl Index<usize> for LinkedSet {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.max_size());
        let mut e = &self.first;
        for _ in 0..index {
            match self.next(*e) {
                None => {
                    panic!("wrong index for LinkedSet!!!")
                }
                Some(ee) => {
                    e = ee;
                }
            }
        }
        e
    }
}

pub struct LinkedSetIter<'a> {
    index: usize,
    value: &'a LinkedSet,
}

impl Iterator for LinkedSetIter<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.index;
        if ret == INVALID {
            return None;
        }
        match self.value.next(self.index) {
            None => {}
            Some(n) => {
                self.index = *n;
            }
        }
        Some(ret)
    }
}
