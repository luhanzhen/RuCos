/*
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

use rucos::domain;
use rucos::variable::domain::domain_trait::DomainTrait;
use rucos::variable::domain::Domain;

#[test]
fn new() {
    let mut dom = domain![1, 2, 3, 4, 76, 43, 23, 43, 21, 34, 55, 33];
    dom.delete_value(23, 3);
    dom.record_limit(3);
    dom.delete_value(43, 4);
    dom.record_limit(4);
    dom.delete_value(21, 5);
    dom.record_limit(5);
    dom.delete_value(33, 10);
    dom.delete_value(2, 10);
    dom.record_limit(10);
    println!("{}", dom);
    // assert_eq!(dom.size(), 11);
    dom.restore_limit(10);
    println!("{}", dom);
    dom.restore_limit(5);

    println!("{}", dom);
    let dom1 = domain!(1 => 3);
    println!("{}", dom1);
    assert_eq!(dom1.size(), 2);

    println!("{}", domain!(1, 3, 4, 5));
    // patterns!(1..3 | 1|3);
    println!("{}", domain!(1 => 3));
}

#[test]
fn remove() {}

#[test]
fn restore() {}
#[test]
fn reduce() {
    let mut dom = domain![10=>100];

    dom.reduce_to_value(19, 1);
    dom.record_limit(1);
    println!("{}", dom);
}
