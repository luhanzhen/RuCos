use futures::executor::ThreadPool;
/**
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/2 13:21
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
use crate::utils::linked_set::LinkedSet;

pub struct Domain {
    elements: LinkedSet,
}

impl Domain {
    fn new(size: usize) -> Self {

        Self {
            elements: LinkedSet::new_with_fill(size),
        }
    }
}
