

/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/24 00:18
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */
use crate::variable::variable::Var;
use std::collections::HashSet;

#[allow(dead_code)]
#[derive(Debug)]
pub struct CoreComponent {
    pub(crate) level: usize,
    pub(crate) decides: usize,
    pub(crate) conflicts: usize,
    pub(crate) propagations: usize,
    pub(crate) filter_counter: usize,
    pub(crate) future_vars: HashSet<Var>,
    pub(crate) past_vars: HashSet<Var>,
}

#[allow(dead_code)]
impl CoreComponent {
    pub fn new(vars: &Vec<Var>) -> Self {
        let mut future = HashSet::with_capacity(vars.len());
        let past = HashSet::with_capacity(vars.len());
        for e in vars.iter() {
            future.insert(e.clone());
        }
        Self {
            level: 0usize,
            decides: 0,
            conflicts: 0,
            propagations: 0,
            filter_counter: 0,
            future_vars: future,
            past_vars: past,
        }
    }
}
