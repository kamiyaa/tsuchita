use crate::context::AppContext;
use crate::error::AppResult;

pub fn reload(context: &mut AppContext, index: usize) -> std::io::Result<()> {
    let options = context.config_ref().display_options_ref().clone();

    Ok(())
}

pub fn reload_list(context: &mut AppContext) -> AppResult<()> {
    Ok(())
}
