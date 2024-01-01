use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

use std::rc::Rc;

/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:   2023/12/24 12:38
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
#[allow(dead_code)]
#[derive(Debug)]
pub struct Seal<T>
where
    T: Clone + Display + Sized,
{
    date: Rc<RefCell<T>>,
}
#[allow(dead_code)]
impl<T> Seal<T>
where
    T: Clone + Display + Sized,
{
    #[inline]
    pub fn new(date: T) -> Self {
        Self {
            date: Rc::new(RefCell::new(date)),
        }
    }
    #[inline]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.date.borrow()
    }
    #[inline]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.date.borrow_mut()
    }
}
#[allow(dead_code)]
impl<T> Clone for Seal<T>
where
    T: Clone + Display + Sized,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            date: Rc::clone(&self.date),
        }
    }
}
#[allow(dead_code)]
impl<T> Display for Seal<T>
where
    T: Clone + Display + Sized,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.date.borrow())
    }
}
impl<T> Deref for Seal<T>
    where
        T: Clone + Display + Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.date.borrow().deref()
    }
}



// #[allow(dead_code)]
// impl<T> Hash for Seal<T>
// where
//     T: Clone + Display ,
// {
//     #[inline]
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.date.borrow().hash(state)
//     }
// }
