use globset::{GlobBuilder, GlobMatcher};

use crate::context::AppContext;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::util::search::SearchPattern;

use super::cursor_move;

pub fn search_glob_fwd(glob: &GlobMatcher) -> Option<usize> {
    None
}
pub fn search_glob_rev(glob: &GlobMatcher) -> Option<usize> {
    None
}

pub fn search_glob(context: &mut AppContext, pattern: &str) -> AppResult<()> {
    Ok(())
}
