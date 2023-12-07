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
        assert_eq!(dom.contains_value(*i), true);
    }
    let dom1 = domain![1=>32];
    for i in 1..32 {
        assert_eq!(dom1.contains_value(i), true);
    }
}

#[test]
fn delete() {
    let mut dom = domain![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33];
    dom.delete_value_at_level(23, 3);
    assert_eq!(dom.contains_value(23), false);
}

#[test]
fn restore() {
    let mut dom = domain![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33];
    dom.delete_value_at_level(23, 3);
    dom.record_limit(3);
    assert_eq!(dom.contains_value(23), false);
    dom.delete_value_at_level(43, 4);
    dom.record_limit(4);
    assert_eq!(dom.contains_value(43), false);
    dom.delete_value_at_level(21, 5);
    dom.record_limit(5);
    assert_eq!(dom.contains_value(21), false);
    dom.delete_value_at_level(33, 10);
    dom.delete_value_at_level(2, 10);
    dom.record_limit(10);
    assert_eq!(dom.contains_value(33), false);
    assert_eq!(dom.contains_value(2), false);
    dom.restore_to_limit(10);
    assert_eq!(dom.contains_value(33), true);
    assert_eq!(dom.contains_value(2), true);
    dom.restore_to_limit(5);
    assert_eq!(dom.contains_value(21), true);
    dom.restore_to_limit(4);
    assert_eq!(dom.contains_value(43), true);
    dom.restore_to_limit(3);
    assert_eq!(dom.contains_value(23), true);

    let mut dom1 = domain![1=>100];
    dom1.delete_value_at_level(23, 3);
    dom1.record_limit(3);
    assert_eq!(dom1.contains_value(23), false);
    dom1.delete_value_at_level(43, 4);
    dom1.record_limit(4);
    assert_eq!(dom1.contains_value(43), false);
    dom1.delete_value_at_level(21, 5);
    dom1.record_limit(5);
    assert_eq!(dom1.contains_value(21), false);
    dom1.delete_value_at_level(33, 10);
    dom1.delete_value_at_level(2, 10);
    dom1.record_limit(10);
    assert_eq!(dom1.contains_value(33), false);
    assert_eq!(dom1.contains_value(2), false);
    dom1.restore_to_limit(10);
    assert_eq!(dom1.contains_value(33), true);
    assert_eq!(dom1.contains_value(2), true);
    dom1.restore_to_limit(5);
    assert_eq!(dom1.contains_value(21), true);
    dom1.restore_to_limit(4);
    assert_eq!(dom.contains_value(43), true);
    dom1.restore_to_limit(3);
    assert_eq!(dom1.contains_value(23), true);
}
#[test]
fn reduce() {
    let mut dom = domain![10=>100];

    assert_eq!(dom.first_idx(), 0);
    assert_eq!(dom.last_idx(), 89);

    dom.reduce_to_value(19, 1);
    dom.record_limit(1);

    assert_eq!(dom.contains_value(19), true);
    assert_eq!(dom.is_single_value(19), true);
    for e in 10..100 {
        if e != 19 {
            assert_eq!(dom.contains_value(e), false);
        }
    }
    dom.restore_to_limit(1);
    for e in 10..100 {
        assert_eq!(dom.contains_value(e), true);
    }
}

#[test]
fn is_boolean() {
    assert_eq!(domain!(0=>2).is_boolean(), true);
    assert_eq!(domain!(0, 1).is_boolean(), true);
    assert_eq!(domain!(0, 1, 2).is_boolean(), false);
    assert_eq!(domain!(0, -1).is_boolean(), false);
}

#[test]
fn update_bound() {
    let mut dom = domain![100=>1000];

    for i in 0..900 {
        assert_eq!(dom.contains_idx(i), true);
    }
    println!("{}", dom);
    dom.update_idx_lower_bound_at_level(150, 1);
    dom.record_limit(1);
    println!("{}", dom);
    for i in 0..150 {
        assert_eq!(dom.contains_idx(i), false);
    }
    for i in 150..900 {
        assert_eq!(dom.contains_idx(i), true);
    }
    dom.update_idx_upper_bound_at_level(876, 2);
    dom.record_limit(2);
    println!("{}", dom);
    for i in 0..150 {
        assert_eq!(dom.contains_idx(i), false);
    }
    for i in 150..876 {
        assert_eq!(dom.contains_idx(i), true);
    }
    for i in 876..900 {
        assert_eq!(dom.contains_idx(i), false);
    }
    dom.restore_to_limit(2);
    dom.restore_to_limit(1);
    for i in 0..900 {
        assert_eq!(dom.contains_idx(i), true);
    }
}

#[test]
fn max_min() {
    let mut dom = domain![10=>100];
    assert_eq!(dom.maximum_value(), 99);
    assert_eq!(dom.minimum_value(), 10);
    assert_eq!(dom.minimum_idx(), 0);
    assert_eq!(dom.maximum_idx(), 89);
    dom.delete_value_at_level(99, 0);
    assert_eq!(dom.maximum_value(), 98);
    assert_eq!(dom.maximum_idx(), 88);
    dom.record_limit(0);
    dom.delete_idx_at_level(0, 1);
    assert_eq!(dom.minimum_value(), 11);
    assert_eq!(dom.minimum_idx(), 1);
    dom.record_limit(1);
    dom.reduce_to_value(32, 2);
    dom.record_limit(2);
    assert_eq!(dom.contains_value(32), true);
    assert_eq!(dom.is_single_value(32), true);
    assert_eq!(dom.maximum_value(), 32);
    assert_eq!(dom.maximum_idx(), 22);
    assert_eq!(dom.which_level_deleted_value(44).unwrap(), 2);
    assert_eq!(dom.which_level_deleted_value(10).unwrap(), 1);
    assert_eq!(dom.which_level_deleted_value(99).unwrap(), 0);
    dom.restore_to_limit(2);
    assert_eq!(dom.minimum_value(), 11);
    assert_eq!(dom.minimum_idx(), 1);
    assert_eq!(dom.maximum_value(), 98);
    assert_eq!(dom.maximum_idx(), 88);

    let mut dom = domain![87, 3, 4, 6, 7, 8, 76, 45, 23, 2, 2, 5, -43, -65, -111];
    assert_eq!(dom.maximum_value(), 87);
    assert_eq!(dom.minimum_value(), -111);
    assert_eq!(dom.minimum_idx(), 0);
    assert_eq!(dom.maximum_idx(), 14);

    dom.delete_value_at_level(-111, 0);
    dom.record_limit(0);
    assert_eq!(dom.minimum_value(), -65);
    assert_eq!(dom.maximum_idx(), 13);
    dom.delete_value_at_level(87, 1);
    dom.record_limit(1);
    assert_eq!(dom.maximum_value(), 76);
    assert_eq!(dom.minimum_idx(), 1);
    dom.restore_to_limit(1);
    dom.restore_to_limit(0);
    assert_eq!(dom.maximum_value(), 87);
    assert_eq!(dom.minimum_value(), -111);
    assert_eq!(dom.minimum_idx(), 0);
    assert_eq!(dom.maximum_idx(), 14);
}
