/**
 * @project_name: RuCos
 *
 * @author: luhan zhen
 *
 * @date:  2023/12/7 16:27
 *
 * @email: zhenluhan@qq.com
 *
 * @version: 1.0
 *
 * @description:
 *
 */

#[allow(dead_code)]
#[derive(Clone)]
pub enum SearchResult {
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
