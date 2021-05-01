use crate::context::AppContext;
use crate::error::AppResult;

use crate::util::sort::SortType;

use super::reload;

pub fn set_sort(context: &mut AppContext, method: SortType) -> AppResult<()> {
    Ok(())
}

pub fn toggle_reverse(context: &mut AppContext) -> AppResult<()> {
    Ok(())
}
