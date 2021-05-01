use crate::context::AppContext;
use crate::error::AppResult;
use crate::util::search::SearchPattern;

pub fn search_next(context: &mut AppContext) -> AppResult<()> {
    if let Some(s) = context.get_search_state() {
        let index = match s {
            SearchPattern::Glob(s) => {}
            SearchPattern::String(s) => {}
        };
    }
    Ok(())
}

pub fn search_prev(context: &mut AppContext) -> AppResult<()> {
    if let Some(s) = context.get_search_state() {
        let index = match s {
            SearchPattern::Glob(s) => {}
            SearchPattern::String(s) => {}
        };
    }
    Ok(())
}
