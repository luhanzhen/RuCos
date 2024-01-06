/* * *
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
 * * */

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct StatusComponent {
    search_status: SearchStates,
    search_result: SearchResult,
}

impl StatusComponent {
    pub(crate) fn new() -> Self {
        Self {
            search_status: SearchStates::Init,
            search_result: SearchResult::Unknown,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum SearchResult {
    Unknown,
    Sat,
    UnSat,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum SearchStates {
    Init,
    Running,
    Found,
    FullExploration,
}
