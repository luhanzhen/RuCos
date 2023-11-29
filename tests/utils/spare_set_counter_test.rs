/*
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/2 13:06
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
use RuCos::utils::set_trait::SetTrait;
use RuCos::utils::spare_set_with_counter::SpareSetCounter;

#[test]
pub fn add() {
    let mut set = SpareSetCounter::new_without_fill(40);
    set.add(33);
    set.add(23);
    set.add(4);
    // println!("{}", set);
    assert_eq!("elements[counter]: 33[1], 23[1], 4[1], \n", set.to_string());
    set.add(33);
    set.add(23);
    set.add(4);
    assert_eq!("elements[counter]: 33[2], 23[2], 4[2], \n", set.to_string());
    set.add(23);
    set.add(6);
    assert_eq!(
        "elements[counter]: 33[2], 23[3], 4[2], 6[1], \n",
        set.to_string()
    );
}

#[test]
pub fn counter() {
    let mut set = SpareSetCounter::new_with_fill(90);
    for i in 0..90usize {
        assert_eq!(set.counter(i), 1);
    }
    for i in 0..90usize {
        set.add(i);
    }
    for i in 0..90usize {
        assert_eq!(set.counter(i), 2);
    }
    set.clear();
    for i in 0..90usize {
        assert_eq!(set.counter(i), 0);
    }
}

#[test]
pub fn clone() {
    let mut set = SpareSetCounter::new_with_fill(90);
    for i in 0..90usize {
        set.delete(i / 2);
    }
    let set1 = set.clone();

    for i in 0..90usize {
        assert_eq!(set1.contains(i / 2), false);
    }
    for i in 45..90usize {
        assert_eq!(set1.contains(i), true);
    }
    set.clear();
    for i in 45..90usize {
        assert_eq!(set1.contains(i), true);
    }
}
