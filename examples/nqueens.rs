use rucos::constraint::constraint::Constraint;
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
use rucos::problem::problem::Problem;
use rucos::solve::solver::solver::Solver;
use rucos::variable::domain::Domain;
use rucos::variable::variable::Var;
use rucos::{all_different, problem, var};

fn n_queens(n: usize) -> Problem {
    let mut problem = problem!();
    for i in 0..n {
        // let var = Var::new(&mut problem, &format!("row_{}", &i), domain![0=>(n as i32)]);
        problem += var!(&format!("row_{}", &i), 0=>(n as i32));
    }

    for i in 0..n {
        for j in i + 1..n {
            problem += all_different!(
                problem[format!("row_{}", i).as_str()].clone(),
                problem[j].clone()
            );
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
