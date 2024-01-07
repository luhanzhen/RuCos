/* * *
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
 * * */
use rucos::prelude::*;

fn n_queens(n: usize) -> Problem {
    let mut problem = problem!("N-Queen");
    for i in 0..n {
        // let var = Var::new(&mut problem, &format!("row_{}", &i), domain![0=>(n as i32)]);
        problem += var!(format!("row_{}", &i).as_str(); 0=>(n as i32));
    }

    for i in 0..n {
        for j in i + 1..n {
            problem += all_different!(&problem[format!("row_{}", i).as_str()], &problem[j]);
        }
    }
    problem
}

fn main() {
    let problem = n_queens(25);
    // let mut solver = problem.solver();
    let mut solver = Solver::from(&problem);

    solver.solve();
    solver.print_statistics();
}
