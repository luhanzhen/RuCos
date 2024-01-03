/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/2 13:21
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */
use rucos::domain;
use rucos::variable::domain::domain_trait::DomainTrait;
use rucos::variable::domain::Domain;

#[test]
fn new() {
    let dom = domain![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33];
    for i in [1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33].iter() {
        assert!(dom.contains_value(*i));
    }
    let dom1 = domain![1=>32];
    for i in 1..32 {
        assert!(dom1.contains_value(i));
    }
}

#[test]
fn delete() {
    let mut dom = domain![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33];
    dom.delete_value_at_level(23, 3);
    assert!(!dom.contains_value(23));
}

#[test]
fn restore() {
    let mut dom = domain![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33];
    dom.delete_value_at_level(23, 3);
    dom.record_limit(3);
    assert!(!dom.contains_value(23));
    dom.delete_value_at_level(43, 4);
    dom.record_limit(4);
    assert!(!dom.contains_value(43));
    dom.delete_value_at_level(21, 5);
    dom.record_limit(5);
    assert!(!dom.contains_value(21));
    dom.delete_value_at_level(33, 10);
    dom.delete_value_at_level(2, 10);
    dom.record_limit(10);
    assert!(!dom.contains_value(33));
    assert!(!dom.contains_value(2));
    dom.restore_to_limit(10);
    assert!(dom.contains_value(33));
    assert!(dom.contains_value(2));
    dom.restore_to_limit(5);
    assert!(dom.contains_value(21));
    dom.restore_to_limit(4);
    assert!(dom.contains_value(43));
    dom.restore_to_limit(3);
    assert!(dom.contains_value(23));

    let mut dom1 = domain![1=>100];
    dom1.delete_value_at_level(23, 3);
    dom1.record_limit(3);
    assert!(!dom1.contains_value(23));
    dom1.delete_value_at_level(43, 4);
    dom1.record_limit(4);
    assert!(!dom1.contains_value(43));
    dom1.delete_value_at_level(21, 5);
    dom1.record_limit(5);
    assert!(!dom1.contains_value(21));
    dom1.delete_value_at_level(33, 10);
    dom1.delete_value_at_level(2, 10);
    dom1.record_limit(10);
    assert!(!dom1.contains_value(33));
    assert!(!dom1.contains_value(2));
    dom1.restore_to_limit(10);
    assert!(dom1.contains_value(33));
    assert!(dom1.contains_value(2));
    dom1.restore_to_limit(5);
    assert!(dom1.contains_value(21));
    dom1.restore_to_limit(4);
    assert!(dom.contains_value(43));
    dom1.restore_to_limit(3);
    assert!(dom1.contains_value(23));
}
#[test]
fn reduce() {
    let mut dom = domain![10=>100];

    assert_eq!(dom.first_idx(), 0);
    assert_eq!(dom.last_idx(), 89);

    dom.reduce_to_value(19, 1);
    dom.record_limit(1);

    assert!(dom.contains_value(19));
    assert!(dom.is_single_value(19));
    for e in 10..100 {
        if e != 19 {
            assert!(!dom.contains_value(e));
        }
    }
    dom.restore_to_limit(1);
    for e in 10..100 {
        assert!(dom.contains_value(e));
    }
}

#[test]
fn is_boolean() {
    assert!(domain!(0=>2).is_boolean());
    assert!(domain!(0, 1).is_boolean());
    assert!(!domain!(0, 1, 2).is_boolean());
    assert!(!domain!(0, -1).is_boolean());
}

#[test]
fn update_bound() {
    let mut dom = domain![100=>1000];

    for i in 0..900 {
        assert!(dom.contains_idx(i));
    }
    println!("{}", dom);
    dom.update_idx_lower_bound_at_level(150, 1);
    dom.record_limit(1);
    println!("{}", dom);
    for i in 0..150 {
        assert!(!dom.contains_idx(i));
    }
    for i in 150..900 {
        assert!(dom.contains_idx(i));
    }
    dom.update_idx_upper_bound_at_level(876, 2);
    dom.record_limit(2);
    println!("{}", dom);
    for i in 0..150 {
        assert!(!dom.contains_idx(i));
    }
    for i in 150..876 {
        assert!(dom.contains_idx(i));
    }
    for i in 876..900 {
        assert!(!dom.contains_idx(i));
    }
    dom.restore_to_limit(2);
    dom.restore_to_limit(1);
    for i in 0..900 {
        assert!(dom.contains_idx(i));
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
    assert!(dom.contains_value(32));
    assert!(dom.is_single_value(32));
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
