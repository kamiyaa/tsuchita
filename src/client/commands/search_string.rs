use crate::context::AppContext;
use crate::error::AppResult;
use crate::util::search::SearchPattern;

use super::cursor_move;

pub fn search_string_fwd(pattern: &str) -> Option<usize> {
    None
}
pub fn search_string_rev(pattern: &str) -> Option<usize> {
    None
}

pub fn search_string(context: &mut AppContext, pattern: &str) -> AppResult<()> {
    Ok(())
}
