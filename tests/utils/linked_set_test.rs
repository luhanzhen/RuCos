/*
 * <p>@project_name: CConstraintSolver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/2 13:58
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

/*
* <p>@project_name: CConstraintSolver
* </p>
* <p>@author: luhan zhen
* </p>
* <p>@date:  2023/11/1 22:00
* </p>
* <p>@email: 940864649@qq.com
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: test for the LinkedSet
* </p>
 */

use CConstraintSolver::utils::linked_set::LinkedSet;
use CConstraintSolver::utils::trait_set::Set;

#[test]
fn add() {
    let mut set = LinkedSet::new(5);
    assert_eq!("elements[ 0, 1, 2, 3, 4, ] //[\n", set.to_string());
    set.delete(4);
    set.delete(3);
    assert_eq!("elements[ 0, 1, 2, ] //[\n", set.to_string());
    set.add(4);
    // assert_eq!("elements[ 0, 1, 2, 4, ] //[\n", set.to_string());
    set.add(3);
    assert_eq!("elements[ 0, 1, 2, 3, 4, ] //[\n", set.to_string());
}

#[test]
fn contain() {
    let set = LinkedSet::new(40);
    for i in 0..40usize {
        assert_eq!(set.contains(i), true);
    }
}

// #[test]
// fn index() {
//     let set = LinkedSet::new(40);
//     for i in 0..40usize {
//         assert_eq!(set[i], i)
//     }
// }

// #[test]
// fn get_position() {
//     let mut set = LinkedSet::new(40);
//     for i in 0..40usize {
//         set.delete(i / 2);
//     }
//     for i in 20..40usize {
//         assert_eq!(set.get_position(i), 40 - i - 1);
//     }
// }

#[test]
fn reduce_to() {
    let mut set = LinkedSet::new(40);
    set.reduce_to(20,2);
    assert_eq!(set.contains(20), true);
    for i in 0..40usize {
        if i != 20 {
            assert_eq!(set.contains(i), false);
        }
    }
}

#[test]
fn delete() {
    let mut set = LinkedSet::new(40);
    set.delete(30);
    assert_eq!(set.contains(30), false);
}

#[test]
#[should_panic]
fn add_over_max() {
    let mut set = LinkedSet::new(40);
    set.add(50);
}

#[test]
fn clone() {
    let mut set = LinkedSet::new(40);
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
    let mut set = LinkedSet::new(40);
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
    let mut set = LinkedSet::new(40);
    assert_eq!(set.size(), 40);
    assert_eq!(set.max_size(), 40);
    for i in 0..40usize {
        set.delete(i / 2);
    }
    assert_eq!(set.size(), 20);
    set.reduce_to(0,2);
    assert_eq!(set.size(), 1);
    assert_eq!(set.max_size(), 40);
}
