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
 * <p>@description: 1.0
* </p>
 */

use CConstraintSolver::utils::spare_set::SpareSet;
use CConstraintSolver::utils::trait_set::Set;
#[test]
fn add() {
    let mut result = SpareSet::new(40);
    result.add(33);
    result.add(23);
    result.add(4);
    // println!("{}", result);
    assert_eq!("elements: 33, 23, 4, \n", result.to_string());
}
#[test]
fn contain() {
    let result = SpareSet::new_with_fill(40);
    for i in 0..40usize {
        assert_eq!(result.contains(i), true);
    }
}

#[test]
fn delete() {
    let mut result = SpareSet::new_with_fill(40);
    result.delete(30);
    assert_eq!(result.contains(30), false);
}
#[test]
#[should_panic]
fn add_over_max() {
    let mut result = SpareSet::new_with_fill(40);
    result.add(50);
}
