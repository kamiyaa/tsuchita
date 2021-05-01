use globset::Glob;

use crate::context::AppContext;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::util::select::SelectOption;

use super::cursor_move;

pub fn select(context: &mut AppContext, pattern: &str, options: &SelectOption) -> AppResult<()> {
    if pattern.is_empty() {
        select_without_pattern(context, options)
    } else {
        select_with_pattern(context, pattern, options)
    }
}

fn select_without_pattern(context: &mut AppContext, options: &SelectOption) -> AppResult<()> {
    Ok(())
}

fn select_with_pattern(
    context: &mut AppContext,
    pattern: &str,
    options: &SelectOption,
) -> AppResult<()> {
    Ok(())
}
