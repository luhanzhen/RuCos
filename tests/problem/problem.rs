/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/11/17 12:29
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */
use rucos::constraint::comparison::all_different::all_different::AllDifferent;
use rucos::constraint::genecric::extension::extension::Extension;
use rucos::domain;
use rucos::variable::domain::Domain;
use rucos::variable::variable::Var;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn add() {
    let mut problem = Default::default();

    let var1 = Var::new(&mut problem, "var1", domain![1=>10]);
    let v1 = Var::new(&mut problem, "v1", domain![7, 43, 22, 33, 2234]);
    let v2 = Var::new(&mut problem, "v2", domain![7, 43, 22, 33, 2234, 43]);

    problem.new_variable(Var::new_without_problem("tt", domain![1=>100]));

    problem.new_constraint(Rc::new(RefCell::new(Extension::new(vec![
        v1.clone(),
        v2.clone(),
    ]))));

    problem.new_constraint(Rc::new(RefCell::new(Extension::new(vec![
        var1.clone(),
        v2.clone(),
    ]))));

    problem.new_constraint(Rc::new(RefCell::new(AllDifferent::new(vec![
        var1.clone(),
        v2.clone(),
        v1.clone(),
    ]))));

    let mut solver = problem.solver();
    solver.delay_construct();
    problem.new_constraint(Rc::new(RefCell::new(Extension::new(vec![
        v1.clone(),
        v1.clone(),
    ]))));

    for c in problem.get_constraints().iter() {
        for p in c.borrow_mut().get_propagators().iter_mut() {
            let _ = p.filter_by_variable(&v1);
        }
    }
    v1.borrow_mut().restore_to_limit(0);
    v2.borrow_mut().restore_to_limit(1);
    println!("v1 {}", v1.borrow().to_string());
    println!("v2 {}", v2.borrow().to_string());
}
