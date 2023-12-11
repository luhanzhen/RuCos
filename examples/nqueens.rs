/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/11 13:09
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use rucos::constraint::comparison::all_different::all_different::AllDifferent;
use rucos::domain;
use rucos::problem::problem::Problem;
use rucos::variable::domain::Domain;
use rucos::variable::variable::Variable;
use std::cell::RefCell;
use std::rc::Rc;

fn n_queens(n: usize) -> Problem {
    let mut problem = Default::default();
    let mut vars = vec![];
    for i in 0..n {
        let var = Variable::new(&mut problem, &format!("row_{}", &i), domain![0=>(n as i32)]);
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
    let problem = n_queens(10);
    let mut solver = problem.solver();
    solver.solve();
    solver.print_statistics();
}
