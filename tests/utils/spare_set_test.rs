/*
* <p>@project_name: constraint_solver
* </p>
* <p>@author: luhan zhen
* </p>
* <p>@date:  2023/11/1 22:00
* </p>
* <p>@email: 940864649@qq.com
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: test for the SpareSet
* </p>
 */

use constraint_solver::utils::spare_set::SpareSet;
use constraint_solver::utils::trait_set::SetTrait;

#[test]
fn add() {
    let mut set = SpareSet::new_without_fill(40);
    set.add(33);
    set.add(23);
    set.add(4);
    // println!("{}", set);
    assert_eq!("elements: 33, 23, 4, \n", set.to_string());
}

#[test]
fn contain() {
    let set = SpareSet::new_with_fill(40);
    for i in 0..40usize {
        assert_eq!(set.contains(i), true);
    }
}

#[test]
fn index() {
    let set = SpareSet::new_with_fill(40);
    for i in 0..40usize {
        assert_eq!(set[i], i)
    }
}

#[test]
fn get_position() {
    let mut set = SpareSet::new_with_fill(40);
    for i in 0..40usize {
        set.delete(i / 2);
    }
    for i in 20..40usize {
        assert_eq!(set.get_position(i), 40 - i - 1);
    }
}

#[test]
fn reduce_to() {
    let mut set = SpareSet::new_with_fill(40);
    set.reduce_to(20);
    assert_eq!(set.contains(20), true);
    for i in 0..40usize {
        if i != 20 {
            assert_eq!(set.contains(i), false);
        }
    }
}

#[test]
fn delete() {
    let mut set = SpareSet::new_with_fill(40);
    set.delete(30);
    assert_eq!(set.contains(30), false);
}

#[test]
#[should_panic]
fn add_over_max() {
    let mut set = SpareSet::new_with_fill(40);
    set.add(50);
}

#[test]
fn clone() {
    let mut set = SpareSet::new_with_fill(40);
    for i in 0..40usize {
        set.delete(i / 2);
    }

    let set1 = set.clone();
    for i in 0..40usize {
        assert_eq!(set1.contains(i / 2), false);
    }
    for i in 20..40usize {
        assert_eq!(set1.contains(i), true);
    }
    set.clear();
    for i in 20..40usize {
        assert_eq!(set1.contains(i), true);
    }
}

#[test]
fn is_empty() {
    let mut set = SpareSet::new_without_fill(40);
    assert_eq!(set.is_empty(), true);
    for i in 0..40usize {
        set.add(i / 2);
        assert_eq!(set.is_empty(), false);
    }
    set.clear();
    assert_eq!(set.is_empty(), true);
}

#[test]
fn size() {
    let mut set = SpareSet::new_with_fill(40);
    assert_eq!(set.size(), 40);
    assert_eq!(set.max_size(), 40);
    for i in 0..40usize {
        set.delete(i / 2);
    }
    assert_eq!(set.size(), 20);
    set.reduce_to(0);
    assert_eq!(set.size(), 1);
    assert_eq!(set.max_size(), 40);
}
