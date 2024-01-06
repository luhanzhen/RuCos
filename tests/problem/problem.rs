use rucos::variable::domain::Domain;
use rucos::variable::variable::Var;
use rucos::{domain, var};

use rucos::constraint::constraint::Constraint;

#[test]
fn add() {
    let mut problem = Default::default();

    let var1 = var!(&mut problem; "var1"; domain![1=>10]);
    let v1 = var!(&mut problem; "v1"; domain![7, 43, 22, 33, 2234]);
    let v2 = var!(&mut problem; "v2"; 7, 43, 22, 33, 2234, 43);

    problem.new_variable(Var::new_without_problem("tt", domain![1=>100]));

    problem.add_constraint(Constraint::new_all_different(vec![v1.clone(), v2.clone()]));

    problem.add_constraint(Constraint::new_extension(vec![var1.clone(), v2.clone()]));

    // problem.new_constraint(Rc::new(RefCell::new(AllDifferent::new(vec![
    //     var1.clone(),
    //     v2.clone(),
    //     v1.clone(),
    // ]))));

    let _ = problem.solver();
    // solver.delay_construct();
    problem.add_constraint(Constraint::new_extension(vec![v1.clone(), v1.clone()]));

    // for c in problem.get_constraints().iter() {
    //     for p in c.borrow_mut().get_propagators().iter_mut() {
    //         let _ = p.filter_by_variable(&v1);
    //     }
    // // }
    // v1.borrow_mut().restore_to_limit(0);
    // v2.borrow_mut().restore_to_limit(1);
    // println!("v1 {}", v1.borrow());
    // println!("v2 {}", v2.borrow());
}
