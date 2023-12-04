/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/2 13:24
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
use rucos::domain;
use rucos::problem::problem::Problem;
use rucos::variable::domain::Domain;
use rucos::variable::variable::Variable;

#[test]
fn new() {
    let problem = Problem::new();
    let v1 = Variable::new(&problem, "v1", &domain![7, 43, 22, 33, 2234]);
    let v2 = Variable::new(&problem, "v2", &domain![7, 43, 22, 33, 2234, 43]);
    assert_eq!(v1.borrow().minimum_value(), 7);
    assert_eq!(v2.borrow().maximum_value(), 2234);
    assert_eq!(problem.borrow().maximum_domain_size(), 6);
}
