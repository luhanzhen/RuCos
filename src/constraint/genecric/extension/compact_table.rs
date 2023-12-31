/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/8 12:18
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use crate::constraint::propagator::PropagatorTrait;
use crate::exception::exception_factory::ExceptionFactory;
use crate::exception::exception_trait::ExceptionTrait;
use crate::exception::ExceptionType;
use crate::variable::variable::Var;

#[allow(dead_code)]
#[derive(Debug)]
pub struct CompactTable {
    scope: Vec<Var>,
    empty_domain_exception: Box<dyn ExceptionTrait>,
}

impl CompactTable {
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
        }
    }
}
#[allow(dead_code)]
impl PropagatorTrait for CompactTable {
    fn initialise(&mut self) {
        todo!()
    }

    fn filter_by_variable(&mut self, _dummy: &Var) -> Result<usize, &Box<dyn ExceptionTrait>> {
        if *_dummy == self.scope[0] {
            println!("before {}", self.scope[0].borrow());
            // match
            self.scope[0].borrow_mut().delete_idx_at_level(0, 0);
            if self.scope[0].borrow().is_empty() {
                return Err(&self.empty_domain_exception);
            }
            self.scope[0].borrow_mut().record_limit(0);
            println!("after {}", self.scope[0].borrow());
        } else {
            println!("before {}", self.scope[1].borrow());

            if self.scope[1].borrow().contains_idx(1) {
                self.scope[1].borrow_mut().delete_idx_at_level(1, 1);
            } else if self.scope[1].borrow().contains_idx(2) {
                self.scope[1].borrow_mut().delete_idx_at_level(2, 1);
            } else if self.scope[1].borrow().contains_idx(3) {
                self.scope[1].borrow_mut().delete_idx_at_level(3, 1);
            }
            if self.scope[1].borrow().is_empty() {
                return Err(&self.empty_domain_exception);
            }
            self.scope[1].borrow_mut().record_limit(1);
            println!("after {}", self.scope[1].borrow());
        }
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
