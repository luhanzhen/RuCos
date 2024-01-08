use crate::constraint::propagator::{PropagationPriority, PropagatorTrait};
use crate::exception::exception_factory::ExceptionFactory;
use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::ExceptionType;
use crate::variable::variable::Var;

/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/22 22:19
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */

#[allow(dead_code)]
#[derive(Debug)]
pub struct BoundConsistency {
    scope: Vec<Var>,
    empty_domain_exception: Box<dyn ExceptionTrait>,
    priority: PropagationPriority,
}

#[allow(dead_code)]
impl BoundConsistency {
    pub fn new(scope: &Vec<Var>) -> Self {
        let mut scope_copy: Vec<Var> = Vec::new();
        scope.iter().for_each(|e| {
            scope_copy.push(e.clone());
        });
        Self {
            scope: scope_copy,
            empty_domain_exception: ExceptionFactory::new(
                ExceptionType::EmptyDomainExceptionType,
                "",
            ),
            priority: PropagationPriority::Unary,
        }
    }
}

#[allow(dead_code)]
impl PropagatorTrait for BoundConsistency {
    fn get_priority(&self) -> &PropagationPriority {
        &self.priority
    }
    fn initialise(&mut self) {
        todo!()
    }

    fn filter_by_variable(&mut self, _dummy: &Var) -> Result<usize, &Box<dyn ExceptionTrait>> {
        Ok(0)
    }

    fn filter_by_arc(
        &mut self,
        _dummy: &Var,
        _value: usize,
    ) -> Result<usize, &Box<dyn ExceptionTrait>> {
        todo!()
    }

    fn is_coarse_grained(&self) -> bool {
        todo!()
    }

    fn is_fine_grained(&self) -> bool {
        todo!()
    }

    fn restore_to_level(&mut self, _level: usize) {
        todo!()
    }
}
