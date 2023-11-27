/*
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/21 20:00
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
use crate::utils::linked_set::LinkedSet;
use std::collections::HashMap;

pub struct DomainRange {
    elements: LinkedSet,
    n_assignment: Box<Vec<usize>>,
    values: Box<Vec<usize>>,
    map: HashMap<i32, usize>,
}
