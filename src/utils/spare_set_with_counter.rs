/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/2 12:42
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

pub struct SpareSetCounter {
    set: SpareSet,
    _counter: Vec<usize>,
}

impl Display for SpareSetCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("elements[counter]: ");
        for i in self.iter() {
            str.push_str(&i.to_string());
            str.push('[');
            str.push_str(&self.counter(i).to_string());
            str.push_str("], ");
        }
        str.push('\n');
        write!(f, "{}", str)
    }
}

impl Clone for SpareSetCounter {
    fn clone(&self) -> Self {
        Self {
            set: self.set.clone(),
            _counter: self._counter.clone(),
        }
    }
}

impl Index<usize> for SpareSetCounter {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.max_size());
        &self.set[index]
    }
}

// impl IndexMut<usize> for SpareSetCounter {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         debug_assert!(index < self.max_size());
//         &mut self.set[index]
//     }
// }

#[allow(dead_code)]
impl SpareSetCounter {
    pub fn new_without_fill(size: usize) -> Self {
        let mut _counter = vec![];
        _counter.resize(size, 0usize);
        Self {
            set: SpareSet::new_without_fill(size),
            _counter,
        }
    }
    pub fn new_with_fill(size: usize) -> Self {
        let mut _counter = vec![];
        _counter.resize(size, 1usize);
        Self {
            set: SpareSet::new_with_fill(size),
            _counter,
        }
    }

    pub fn iter(&self) -> SpareSetIter {
        self.set.iter()
    }

    pub fn counter(&self, ele: usize) -> usize {
        debug_assert!(ele < self.max_size());
        self._counter[ele]
    }

    pub fn reduce_to(&mut self, ele: usize) {
        self.set.reduce_to(ele)
    }
}

impl SetTrait<usize> for SpareSetCounter {
    #[inline]
    fn add(&mut self, ele: usize) {
        if self.contains(ele) {
            self._counter[ele] += 1
        } else {
            self.set.add(ele);
            self._counter[ele] = 1
        }
    }
    #[inline]
    fn fill(&mut self) {
        self.set.fill()
    }
    #[inline]
    fn delete(&mut self, ele: usize) {
        if self.set.contains(ele) {
            self._counter[ele] -= 1;
            self.set.delete(ele)
        }
    }
    #[inline]
    fn contains(&self, ele: usize) -> bool {
        self.set.contains(ele)
    }
    #[inline]
    fn size(&self) -> usize {
        self.set.size()
    }
    #[inline]
    fn clear(&mut self) {
        self.set.clear();
        self._counter.fill(0usize);
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
    #[inline]
    fn max_size(&self) -> usize {
        self.set.max_size()
    }
}
