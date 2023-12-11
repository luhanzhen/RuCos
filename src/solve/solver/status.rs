/*
 * <p>@project_name: RuCos
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/12/7 16:27
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
#[allow(dead_code)]
#[derive(Clone)]
pub enum SearchResult {
    Init,
    Unknown,
    Sat,
    UnSat,
}
#[allow(dead_code)]
#[derive(Clone)]
pub enum SearchStates {
    Init,
    Running,
    Found,
    FullExploration,
}
