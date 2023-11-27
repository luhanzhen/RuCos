/*
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/2 9:54
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

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
