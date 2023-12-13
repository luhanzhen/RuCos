/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/12 18:45
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn 
 * </p>
 * <p>@version: 1.0
 * </p>
  * <p>@description: 
 * </p>
 */

#![cfg_attr(feature = "unstable", feature(test))]
#[deny(soft_unstable)]
extern crate test;

use test::{black_box, Bencher};






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
    let problem = n_queens(200);
    let mut solver = problem.solver();
    solver.solve();
    solver.print_statistics();
}

 