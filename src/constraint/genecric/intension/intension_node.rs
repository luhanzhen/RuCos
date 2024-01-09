/***
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:   2024/1/8 20:24
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 **/

use crate::prelude::Var;
use std::cmp::Ordering;
use std::ops::*;
#[allow(dead_code)]
enum IntensionNode {
    Var(Var),
    Node(Box<IntensionNode>),
}

impl PartialEq<Self> for IntensionNode {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for IntensionNode {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        todo!()
    }

    fn lt(&self, _other: &Self) -> bool {
        todo!()
    }

    fn le(&self, _other: &Self) -> bool {
        todo!()
    }

    fn gt(&self, _other: &Self) -> bool {
        todo!()
    }

    fn ge(&self, _other: &Self) -> bool {
        todo!()
    }
}
impl Add<Self> for IntensionNode {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Rem<Self> for IntensionNode {
    type Output = Self;

    fn rem(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Neg for IntensionNode {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
impl Mul<Self> for IntensionNode {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Sub<Self> for IntensionNode {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
