use crate::context::AppContext;
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::ui::TuiBackend;
use crate::util::select::SelectOption;
use crate::util::sort::SortType;

use super::*;

#[derive(Clone, Debug)]
pub enum KeyCommand {
    Quit,
    ForceQuit,

    CursorMoveUp(usize),
    CursorMoveDown(usize),
    CursorMoveLeft,
    CursorMoveRight,
    CursorMoveHome,
    CursorMoveEnd,
    CursorMovePageUp,
    CursorMovePageDown,

    ReloadList,

    SearchGlob(String),
    SearchString(String),
    SearchNext,
    SearchPrev,

    Select(String, SelectOption),

    Sort(SortType),
    SortReverse,
}

impl KeyCommand {
    pub fn command(&self) -> &'static str {
        match self {
            Self::Quit => "quit",
            Self::ForceQuit => "force_quit",

            Self::CursorMoveUp(_) => "cursor_move_up",
            Self::CursorMoveDown(_) => "cursor_move_down",
            Self::CursorMoveLeft => "cursor_move_left",
            Self::CursorMoveRight => "cursor_move_right",
            Self::CursorMoveHome => "cursor_move_home",
            Self::CursorMoveEnd => "cursor_move_end",
            Self::CursorMovePageUp => "cursor_move_page_up",
            Self::CursorMovePageDown => "cursor_move_page_down",

            Self::ReloadList => "reload_list",

            Self::SearchString(_) => "search",
            Self::SearchGlob(_) => "search_glob",
            Self::SearchNext => "search_next",
            Self::SearchPrev => "search_prev",

            Self::Select(_, _) => "select",

            Self::Sort(_) => "sort",
            Self::SortReverse => "sort reverse",
        }
    }

    pub fn parse_command(s: &str) -> AppResult<Self> {
        let (command, arg) = match s.find(' ') {
            Some(i) => (&s[..i], s[i..].trim_start()),
            None => (s, ""),
        };

        match command {
            "cursor_move_home" => Ok(Self::CursorMoveHome),
            "cursor_move_end" => Ok(Self::CursorMoveEnd),
            "cursor_move_page_up" => Ok(Self::CursorMovePageUp),
            "cursor_move_page_down" => Ok(Self::CursorMovePageDown),
            "cursor_move_left" => Ok(Self::CursorMoveLeft),
            "cursor_move_right" => Ok(Self::CursorMoveRight),
            "cursor_move_down" => match arg {
                "" => Ok(Self::CursorMoveDown(1)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::CursorMoveDown(s)),
                    Err(e) => Err(AppError::new(AppErrorKind::ParseError, e.to_string())),
                },
            },
            "cursor_move_up" => match arg {
                "" => Ok(Self::CursorMoveUp(1)),
                arg => match arg.trim().parse::<usize>() {
                    Ok(s) => Ok(Self::CursorMoveUp(s)),
                    Err(e) => Err(AppError::new(AppErrorKind::ParseError, e.to_string())),
                },
            },
            "force_quit" => Ok(Self::ForceQuit),
            "quit" => Ok(Self::Quit),
            "reload_list" => Ok(Self::ReloadList),

            "search_glob" => match arg {
                "" => Err(AppError::new(
                    AppErrorKind::IoInvalidData,
                    format!("{}: Expected 1, got 0", command),
                )),
                arg => Ok(Self::SearchGlob(arg.to_string())),
            },
            "search_next" => Ok(Self::SearchNext),
            "search_prev" => Ok(Self::SearchPrev),
            "select" => {
                let mut options = SelectOption::default();
                let mut pattern = "";
                match shell_words::split(arg) {
                    Ok(args) => {
                        for arg in args.iter() {
                            match arg.as_str() {
                                "--toggle=true" => options.toggle = true,
                                "--all=true" => options.all = true,
                                "--toggle=false" => options.toggle = false,
                                "--all=false" => options.all = false,
                                "--deselect=true" => options.reverse = true,
                                "--deselect=false" => options.reverse = false,
                                s => pattern = s,
                            }
                        }
                        Ok(Self::Select(pattern.to_string(), options))
                    }
                    Err(e) => Err(AppError::new(
                        AppErrorKind::IoInvalidData,
                        format!("{}: {}", arg, e),
                    )),
                }
            }
            "sort" => match arg {
                "reverse" => Ok(Self::SortReverse),
                arg => match SortType::parse(arg) {
                    Some(s) => Ok(Self::Sort(s)),
                    None => Err(AppError::new(
                        AppErrorKind::IoInvalidData,
                        format!("sort: Unknown option '{}'", arg),
                    )),
                },
            },
            inp => Err(AppError::new(
                AppErrorKind::UnknownCommand,
                format!("Unknown command: {}", inp),
            )),
        }
    }

    pub fn execute(&self, context: &mut AppContext, backend: &mut TuiBackend) -> AppResult<()> {
        match &*self {
            Self::CursorMoveUp(u) => cursor_move::up(context, *u),
            Self::CursorMoveDown(u) => cursor_move::down(context, *u),
            Self::CursorMoveLeft => cursor_move::left(context),
            Self::CursorMoveRight => cursor_move::right(context),
            Self::CursorMoveHome => cursor_move::home(context),
            Self::CursorMoveEnd => cursor_move::end(context),
            Self::CursorMovePageUp => cursor_move::page_up(context, backend),
            Self::CursorMovePageDown => cursor_move::page_down(context, backend),

            Self::Quit => quit::quit(context),
            Self::ForceQuit => quit::force_quit(context),

            /*
                        Self::ReloadList => reload::reload_list(context),

                        Self::SearchGlob(pattern) => search_glob::search_glob(context, pattern.as_str()),
                        Self::SearchString(pattern) => search_string::search_string(context, pattern.as_str()),
                        Self::SearchNext => search::search_next(context),
                        Self::SearchPrev => search::search_prev(context),
                        Self::Select(pattern, options) => {
                            selection::select(context, pattern.as_str(), &options)
                        }
                        Self::Sort(t) => sort::set_sort(context, *t),
                        Self::SortReverse => sort::toggle_reverse(context),
            */
            _ => Ok(()),
        }
    }
}

impl std::fmt::Display for KeyCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Self::CursorMoveUp(i) => write!(f, "{} {}", self.command(), i),
            Self::CursorMoveDown(i) => write!(f, "{} {}", self.command(), i),

            Self::SearchGlob(s) => write!(f, "{} {}", self.command(), s),
            Self::SearchString(s) => write!(f, "{} {}", self.command(), s),
            Self::Select(pattern, options) => {
                write!(f, "{} {} {}", self.command(), pattern, options)
            }
            Self::Sort(t) => write!(f, "{} {}", self.command(), t),
            _ => write!(f, "{}", self.command()),
        }
    }
}
