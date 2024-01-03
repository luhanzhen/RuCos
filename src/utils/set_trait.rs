/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/2 9:54
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */
use std::fmt::Display;

pub trait SetTrait<T>: Clone + Display {
    fn add(&mut self, ele: T);

    fn fill(&mut self);

    fn delete(&mut self, ele: T);

    fn contains(&self, ele: usize) -> bool;

    fn size(&self) -> usize;

    fn clear(&mut self);

    fn is_empty(&self) -> bool;

    fn max_size(&self) -> usize;
}
