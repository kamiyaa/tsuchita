use std::collections::{hash_map::Entry, HashMap};

use crate::context::AppContext;
use crate::error::AppResult;
use crate::fs::{EntryType, TsuchitaEntry, TsuchitaList, TsuchitaMessage};
use crate::tree::DbusTreeTrait;
use crate::ui::TuiBackend;

pub fn cursor_move(mut new_index: usize, context: &mut AppContext) -> AppResult<()> {
    if let Some(curr_list) = context.curr_list_mut() {
        if !curr_list.is_empty() {
            let dir_len = curr_list.len();
            if new_index >= dir_len {
                new_index = dir_len - 1;
            }
            curr_list.index = new_index;
        }
    }

    if context.path_ref() == "/" {
        let source_name = match context.curr_list_ref() {
            Some(curr_list) => match curr_list.curr_entry_ref() {
                Some(entry) => Some(entry.name().to_string()),
                None => None,
            },
            None => None,
        };
        if let Some(source) = source_name {
            let url = context.config_ref().server_ref().url.clone();
            let display_options = context
                .config_ref()
                .client_ref()
                .display_options_ref()
                .clone();
            match context.tree_mut().entry(source.clone()) {
                Entry::Occupied(entry) => {
                    if (entry.get().needs_update()) {
                        context.tree_mut().fetch_messages(
                            url.as_str(),
                            source.as_str(),
                            &display_options,
                        );
                    }
                }
                Entry::Vacant(_) => {
                    context.tree_mut().fetch_messages(
                        url.as_str(),
                        source.as_str(),
                        &display_options,
                    );
                }
            }
        }
    }
    Ok(())
}

pub fn left(context: &mut AppContext) -> AppResult<()> {
    if context.path_ref() != "/" {
        context.set_path("/".to_string());
    }
    Ok(())
}

pub fn right(context: &mut AppContext) -> AppResult<()> {
    let source_name: Option<String> = match context
        .curr_list_ref()
        .and_then(|list| list.curr_entry_ref())
    {
        Some(entry) => match entry.get_type() {
            EntryType::Source => Some(entry.name().to_string()),
            _ => None,
        },
        None => None,
    };

    if let Some(source) = source_name {
        context.set_path(source);
    }
    Ok(())
}

pub fn up(context: &mut AppContext, u: usize) -> AppResult<()> {
    let movement = match context.curr_list_ref() {
        Some(curr_list) => Some(if curr_list.index > u {
            curr_list.index - u
        } else {
            0
        }),
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn down(context: &mut AppContext, u: usize) -> AppResult<()> {
    let movement = match context.curr_list_ref() {
        Some(curr_list) => Some(curr_list.index + u),
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn home(context: &mut AppContext) -> AppResult<()> {
    let movement: Option<usize> = match context.curr_list_ref() {
        Some(curr_list) => {
            let len = curr_list.len();
            if len == 0 {
                None
            } else {
                Some(0)
            }
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn end(context: &mut AppContext) -> AppResult<()> {
    let movement: Option<usize> = match context.curr_list_ref() {
        Some(curr_list) => {
            let len = curr_list.len();
            if len == 0 {
                None
            } else {
                Some(len - 1)
            }
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn page_up(context: &mut AppContext, backend: &mut TuiBackend) -> AppResult<()> {
    let half_page = {
        match backend.terminal.as_ref().unwrap().size() {
            Ok(rect) => rect.height as usize - 2,
            _ => 10,
        }
    };

    let movement = match context.curr_list_ref() {
        Some(curr_list) => {
            Some(curr_list.index).map(|idx| if idx > half_page { idx - half_page } else { 0 })
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}

pub fn page_down(context: &mut AppContext, backend: &mut TuiBackend) -> AppResult<()> {
    let half_page = {
        match backend.terminal.as_ref().unwrap().size() {
            Ok(rect) => rect.height as usize - 2,
            _ => 10,
        }
    };

    let movement = match context.curr_list_ref() {
        Some(curr_list) => {
            let dir_len = curr_list.len();
            Some(curr_list.index).map(|idx| {
                if idx + half_page > dir_len - 1 {
                    dir_len - 1
                } else {
                    idx + half_page
                }
            })
        }
        None => None,
    };

    if let Some(s) = movement {
        cursor_move(s, context)?;
    }
    Ok(())
}
