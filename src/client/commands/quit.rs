use crate::context::AppContext;
use crate::error::AppResult;

pub fn quit(context: &mut AppContext) -> AppResult<()> {
    context.exit = true;
    Ok(())
}

pub fn force_quit(context: &mut AppContext) -> AppResult<()> {
    context.exit = true;
    Ok(())
}
