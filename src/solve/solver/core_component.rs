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
use crate::prelude::Constraint;
use crate::variable::variable::Var;
use rand::random;
use std::collections::HashSet;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoreComponent {
    pub(crate) level_max: usize,
    pub(crate) level: usize,
    pub(crate) decides: usize,
    pub(crate) conflicts: usize,
    pub(crate) propagations: usize,
    pub(crate) filter_counter: usize,
    pub(crate) future_vars: HashSet<Var>,
    pub(crate) past_vars: HashSet<Var>,
    pub(crate) variables: Vec<Var>,
    pub(crate) constraints: Vec<Constraint>,
}

#[allow(dead_code)]
impl CoreComponent {
    pub fn new(vars: Vec<Var>, constraint: Vec<Constraint>) -> Self {
        let mut future = HashSet::with_capacity(vars.len());
        let past = HashSet::with_capacity(vars.len());
        for e in vars.iter() {
            future.insert(e.clone());
        }
        Self {
            level_max: vars.len(),
            level: 0usize,
            decides: 0,
            conflicts: 0,
            propagations: 0,
            filter_counter: 0,
            future_vars: future,
            past_vars: past,
            variables: vars,
            constraints: constraint,
        }
    }
}

#[allow(dead_code)]
impl CoreComponent {
    pub(crate) fn do_something_constraint<P>(&self, mut function: P)
    where
        P: FnMut(&Constraint),
    {
        for constraint in self.constraints.iter() {
            function(constraint)
        }
    }
    pub(crate) fn add_constraint_to_variable_scoped(&self) {
        for cons in self.constraints.iter() {
            for var in cons.borrow().get_scope().iter() {
                var.borrow_mut().belongs_to_the_constraint(cons.clone())
            }
        }
    }

    pub(crate) fn do_something_variables<P>(&self, mut function: P)
    where
        P: FnMut(&Var),
    {
        for var in self.variables.iter() {
            function(var)
        }
    }

    pub(crate) fn get_all_variables(&self) -> &Vec<Var> {
        &self.variables
    }
    pub(crate) fn get_variables_size(&self) -> usize {
        self.variables.len()
    }

    pub(crate) fn shuffle_variables(&mut self) {
        for i in (1..=self.variables.len()).rev() {
            self.variables.swap(i - 1, random::<usize>() % i);
        }
    }

    pub(crate) fn maximum_arity(&self) -> usize {
        let mut max = usize::MIN;
        for con in self.constraints.iter() {
            let m = con.borrow().get_arity();
            if max < m {
                max = m;
            }
        }
        max
    }

    pub(crate) fn minimum_arity(&self) -> usize {
        let mut min = usize::MIN;
        for con in self.constraints.iter() {
            let m = con.borrow().get_arity();
            if min > m {
                min = m;
            }
        }
        min
    }

    pub(crate) fn maximum_domain_size(&self) -> usize {
        let mut max = usize::MIN;
        for var in self.variables.iter() {
            let m = var.borrow().domain_size();
            if max < m {
                max = m;
            }
        }
        max
    }

    pub(crate) fn minimum_domain_size(&self) -> usize {
        let mut min = usize::MAX;
        for var in self.variables.iter() {
            let m = var.borrow().domain_size();
            if min > m {
                min = m;
            }
        }
        min
    }
}
