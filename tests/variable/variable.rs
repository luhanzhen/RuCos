use rucos::problem::problem::Problem;
use rucos::variable::domain::Domain;
use rucos::variable::variable::Var;

/* * *
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/2 13:24
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 * * */
use rucos::{bool, domain, var};

#[test]
fn new() {
    let mut problem = Problem::new();
    let v1 = var!(&mut problem; "v1"; domain![7, 43, 22, 33, 2234]);
    let v2 = var!(&mut problem; "v2"; domain![7, 43, 22, 33, 2234, 43]);
    let v3 = var!(&mut problem; "v3"; 7, 43, 22, 33, 2234, 43);

    assert_eq!(problem["v1"], v1);
    assert_eq!(problem["v2"], v2);
    assert_eq!(problem["v3"], v3);
    assert_eq!(problem[0], v1);
    assert_eq!(problem[1], v2);
    assert_eq!(problem[2], v3);

    assert_eq!(v3.borrow().minimum_value(), 7);
    assert_eq!(v3.borrow().maximum_value(), 2234);
    assert_eq!(v1.borrow().minimum_value(), 7);
    assert_eq!(v2.borrow().maximum_value(), 2234);

    problem += var!("v4"; 7=> 43);
    problem += var!("v5";7, 54, 65, 43);
    problem += var!("v6"; 7=> 43);

    problem += bool!("v5");
    problem += bool!("v6");
    problem += bool!("vbool");
    problem += bool!("vbooll");
    problem += bool!();

    assert_eq!(problem["vbool"].borrow().maximum_idx(), 1);
    assert_eq!(problem["vbool"].borrow().minimum_value(), 0);
}
