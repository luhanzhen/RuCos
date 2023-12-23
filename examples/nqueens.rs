/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/11 13:09
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */


use rucos::constraint::comparison::all_different::all_different::AllDifferent;
use rucos::{domain,var};
use rucos::problem::problem::Problem;
use rucos::solve::solver::solver::Solver;
use rucos::variable::domain::Domain;
use rucos::variable::variable::Var;
use std::cell::RefCell;
use std::rc::Rc;

fn n_queens(n: usize) -> Problem {
    let mut problem = Default::default();
    let mut vars = vec![];
    for i in 0..n {
        // let var = Var::new(&mut problem, &format!("row_{}", &i), domain![0=>(n as i32)]);
        let var = var!(&mut problem, &format!("row_{}", &i), domain![0=>(n as i32)]);
        vars.push(var);
    }

    for i in 0..n {
        for j in i + 1..n {
            problem.new_constraint(Rc::new(RefCell::new(AllDifferent::new(vec![
                vars[i].clone(),
                vars[j].clone(),
            ]))));
        }
    }

    problem
}

fn main() {
    let problem = n_queens(200);
    // let mut solver = problem.solver();
    let mut solver = Solver::from(&problem);

    solver.solve();
    solver.print_statistics();
}
