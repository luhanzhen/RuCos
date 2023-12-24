/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/2 13:58
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */

/**
* @project_name: RuCos
*
* @author: luhan zhen
*
* @date:  2023/11/1 22:00
*
* @email: 940864649@qq.com
*
* @version: 1.0
*
 * @description: test for the LinkedSet
*
 */
use rucos::utils::linked_set::LinkedSet;
use rucos::utils::set_trait::SetTrait;

#[test]
fn add() {
    let mut set = LinkedSet::new_with_fill(5);
    assert_eq!("elements[ 0, 1, 2, 3, 4, ] deleted[]\n", set.to_string());
    set.delete_at_level(3, 0);
    set.delete_at_level(4, 1);
    assert_eq!("elements[ 0, 1, 2, ] deleted[3, 4, ]\n", set.to_string());
    set.restore_last_dropped();
    assert_eq!("elements[ 0, 1, 2, 4, ] deleted[3, ]\n", set.to_string());
    set.restore_last_dropped();
    assert_eq!("elements[ 0, 1, 2, 3, 4, ] deleted[]\n", set.to_string());
}

#[test]
fn contain() {
    let mut set = LinkedSet::new_with_fill(40);
    set.delete_at_level(3, 0);
    set.delete_at_level(4, 4);
    for i in 0..40usize {
        if i != 3 && i != 4 {
            assert!(set.contains(i));
        }
    }
}

#[test]
fn reduce_to() {
    let mut set = LinkedSet::new_with_fill(40);
    set.reduce_to(20, 2);
    assert!(set.contains(20));
    // println!("{}",set);
    assert_eq!("elements[ 20, ] deleted[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, ]\n", set.to_string());
    for i in 0..40usize {
        if i != 20 {
            assert!(!set.contains(i));
        }
    }
}

#[test]
fn delete() {
    let mut set = LinkedSet::new_with_fill(40);
    for i in 0..40usize {
        if i % 2 == 0 {
            set.delete_at_level(i, i / 2);
        }
    }
    for i in 0..40usize {
        if i % 2 == 1 {
            assert!(set.contains(i));
        }
    }
}

#[test]
#[should_panic]
fn add_over_max() {
    let mut set = LinkedSet::new_with_fill(40);
    set.delete_at_level(50, 0);
}

#[test]
fn clone() {
    let mut set = LinkedSet::new_with_fill(40);
    for i in 0..40usize {
        set.delete_at_level(i / 2, 0);
    }
    let set1 = set.clone();
    for i in 0..40usize {
        assert!(!set1.contains(i / 2));
    }
    for i in 20..40usize {
        assert!(set1.contains(i));
    }

    for i in 0..20usize {
        assert!(!set1.contains(i));
    }
}

#[test]
fn is_empty() {
    let mut set = LinkedSet::new_with_fill(40);
    assert!(!set.is_empty());
    for i in 0..39usize {
        set.delete_at_level(i, 0);
        assert!(!set.is_empty());
    }
    set.delete_at_level(39, 0);
    assert!(set.is_empty());
    set.restore_last_dropped();
    assert!(!set.is_empty());
}

#[test]
fn size() {
    let mut set = LinkedSet::new_with_fill(40);
    assert_eq!(set.size(), 40);
    assert_eq!(set.max_size(), 40);
    for i in 0..40usize {
        set.delete_at_level(i / 2, i / 2);
    }
    assert_eq!(set.size(), 20);
    assert_eq!(set.reduce_to(23, 33), 20);
    // println!("{}",set);
    assert_eq!(set.size(), 1);
    assert_eq!(set.max_size(), 40);
}
