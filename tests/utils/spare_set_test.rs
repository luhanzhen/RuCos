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
 * @description: test for the SpareSet
*
 */
use rucos::utils::set_trait::SetTrait;
use rucos::utils::spare_set::SpareSet;

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
        assert!(set.contains(i));
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
    assert!(set.contains(20));
    for i in 0..40usize {
        if i != 20 {
            assert!(!set.contains(i));
        }
    }
}

#[test]
fn delete() {
    let mut set = SpareSet::new_with_fill(40);
    set.delete(30);
    assert!(!set.contains(30));
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
        assert!(!set1.contains(i / 2));
    }
    for i in 20..40usize {
        assert!(set1.contains(i));
    }
    set.clear();
    for i in 20..40usize {
        assert!(set1.contains(i));
    }
}

#[test]
fn is_empty() {
    let mut set = SpareSet::new_without_fill(40);
    assert!(set.is_empty());
    for i in 0..40usize {
        set.add(i / 2);
        assert!(!set.is_empty());
    }
    set.clear();
    assert!(set.is_empty());
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
