/*
 * <p>@project_name: constraint_solver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/2 13:21
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

use RuCos::variable::domain::domain_trait::DomainTrait;
use RuCos::variable::domain::domain_values::DomainValues;

#[test]
fn add() {
    let domain = DomainValues::new(12);

    assert_eq!(domain.size(), 12)
}
