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

#[cfg(test)]
mod tests {
    use CConstraintSolver::utils::spare_set::SpareSet;

    #[test]
    fn it_works() {
        let mut result = SpareSet::new(40);
        result.add(33);
        result.add(23);
        result.add(4);
        println!("{}", result);
        // assert_eq!(result, 4);
    }
}
