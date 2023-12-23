/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/2 13:13
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::utils::set_trait::SetTrait;
use crate::utils::spare_set::{SpareSet, SpareSetIter};
use std::fmt::{Display, Formatter};
use std::ops::Index;

const INVALID: usize = usize::MAX;

struct SpareSetMultiLevel {
    set: SpareSet,
    limits: Vec<usize>,
}

#[allow(dead_code)]
impl SpareSetMultiLevel {
    pub fn iter(&self) -> SpareSetIter {
        self.set.iter()
    }

    pub fn new_without_fill(size: usize) -> Self {
        Self {
            set: SpareSet::new_without_fill(size),
            limits: vec![],
        }
    }
    pub fn new_with_fill(size: usize) -> Self {
        Self {
            set: SpareSet::new_with_fill(size),
            limits: vec![],
        }
    }
    pub fn record_limit(&mut self, level: usize) {
        if level >= self.limits.len() {
            self.limits.resize(level + 1, INVALID);
        }
        debug_assert!(self.limits[level] == INVALID);
        self.limits[level] = *self.set.limit()
    }
    pub fn is_limit_recorded_at_level(&self, level: usize) -> bool {
        level < self.limits.len() && self.limits[level] != INVALID
    }

    pub fn restore_limit(&mut self, level: usize) {
        debug_assert!(self.limits[level] != INVALID);
        *self.set.limit() = self.limits[level];
        self.limits[level] = INVALID
    }
}

impl SetTrait<usize> for SpareSetMultiLevel {
    fn add(&mut self, ele: usize) {
        self.set.add(ele)
    }

    fn fill(&mut self) {
        self.set.fill()
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
        self.set.clear();
    }

    fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    fn max_size(&self) -> usize {
        self.set.max_size()
    }
}

impl Display for SpareSetMultiLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("elements[limits]: ");
        for i in self.iter() {
            str.push_str(&i.to_string());
            // str.push_str("[");
            // str.push_str(&*self.limits(i).to_string());
            str.push_str(", ");
        }
        str.push('\n');
        write!(f, "{}", str)
    }
}

impl Index<usize> for SpareSetMultiLevel {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.max_size());
        &self.set[index]
    }
}

impl Clone for SpareSetMultiLevel {
    fn clone(&self) -> Self {
        Self {
            set: self.set.clone(),
            limits: self.limits.clone(),
        }
    }
}
