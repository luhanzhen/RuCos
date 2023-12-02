/*
 * <p>@project_name: RuCos
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

use rucos::domain;
use rucos::variable::domain::domain_trait::DomainTrait;
use rucos::variable::domain::Domain;

#[test]
fn new() {
    let dom = domain![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33];
    for i in vec![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33].iter() {
        assert_eq!(dom.contain_value(*i), true);
    }
    let dom1 = domain![1=>32];
    for i in 1..32 {
        assert_eq!(dom1.contain_value(i), true);
    }
}

#[test]
fn delete() {
    let mut dom = domain![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33];
    dom.delete_value(23, 3);
    assert_eq!(dom.contain_value(23), false);
}

#[test]
fn restore() {
    let mut dom = domain![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33];
    dom.delete_value(23, 3);
    dom.record_limit(3);
    assert_eq!(dom.contain_value(23), false);
    dom.delete_value(43, 4);
    dom.record_limit(4);
    assert_eq!(dom.contain_value(43), false);
    dom.delete_value(21, 5);
    dom.record_limit(5);
    assert_eq!(dom.contain_value(21), false);
    dom.delete_value(33, 10);
    dom.delete_value(2, 10);
    dom.record_limit(10);
    assert_eq!(dom.contain_value(33), false);
    assert_eq!(dom.contain_value(2), false);
    dom.restore_limit(10);
    assert_eq!(dom.contain_value(33), true);
    assert_eq!(dom.contain_value(2), true);
    dom.restore_limit(5);
    assert_eq!(dom.contain_value(21), true);
    dom.restore_limit(4);
    assert_eq!(dom.contain_value(43), true);
    dom.restore_limit(3);
    assert_eq!(dom.contain_value(23), true);

    let mut dom1 = domain![1=>100];
    dom1.delete_value(23, 3);
    dom1.record_limit(3);
    assert_eq!(dom1.contain_value(23), false);
    dom1.delete_value(43, 4);
    dom1.record_limit(4);
    assert_eq!(dom1.contain_value(43), false);
    dom1.delete_value(21, 5);
    dom1.record_limit(5);
    assert_eq!(dom1.contain_value(21), false);
    dom1.delete_value(33, 10);
    dom1.delete_value(2, 10);
    dom1.record_limit(10);
    assert_eq!(dom1.contain_value(33), false);
    assert_eq!(dom1.contain_value(2), false);
    dom1.restore_limit(10);
    assert_eq!(dom1.contain_value(33), true);
    assert_eq!(dom1.contain_value(2), true);
    dom1.restore_limit(5);
    assert_eq!(dom1.contain_value(21), true);
    dom1.restore_limit(4);
    assert_eq!(dom.contain_value(43), true);
    dom1.restore_limit(3);
    assert_eq!(dom1.contain_value(23), true);
}
#[test]
fn reduce() {
    let mut dom = domain![10=>100];
    assert_eq!(dom.maximum(), 99);
    assert_eq!(dom.minimum(), 10);
    assert_eq!(dom.first_idx(), 0);
    assert_eq!(dom.last_idx(), 89);

    dom.reduce_to_value(19, 1);
    dom.record_limit(1);

    assert_eq!(dom.contain_value(19), true);
    assert_eq!(dom.is_single_value(19), true);
    for e in 10..100 {
        if e != 19 {
            assert_eq!(dom.contain_value(e), false);
        }
    }
    dom.restore_limit(1);
    for e in 10..100 {
        assert_eq!(dom.contain_value(e), true);
    }
}

#[test]
fn is_boolean() {
    assert_eq!(domain!(0=>2).is_boolean(), true);
    assert_eq!(domain!(0, 1).is_boolean(), true);
    assert_eq!(domain!(0, 1, 2).is_boolean(), false);
    assert_eq!(domain!(0, -1).is_boolean(), false);
}
