use std::cmp;
use std::time;

use serde_derive::Deserialize;

use crate::fs::{EntryType, TsuchitaEntry};

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum SortType {
    Lexical,
    Mtime,
    Natural,
}

impl SortType {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "lexical" => Some(SortType::Lexical),
            "mtime" => Some(SortType::Mtime),
            "natural" => Some(SortType::Natural),
            _ => None,
        }
    }
    pub const fn as_str(&self) -> &str {
        match *self {
            SortType::Lexical => "lexical",
            SortType::Mtime => "mtime",
            SortType::Natural => "natural",
        }
    }
}

impl std::fmt::Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct SortOption {
    pub case_sensitive: bool,
    pub reverse: bool,
    pub sort_method: SortType,
}

impl SortOption {
    pub fn compare(&self, f1: &TsuchitaEntry, f2: &TsuchitaEntry) -> cmp::Ordering {
        let mut res = match self.sort_method {
            SortType::Lexical => {
                let f1_name = f1.name();
                let f2_name = f2.name();
                if self.case_sensitive {
                    f1_name.cmp(&f2_name)
                } else {
                    let f1_name = f1_name.to_lowercase();
                    let f2_name = f2_name.to_lowercase();
                    f1_name.cmp(&f2_name)
                }
            }
            SortType::Natural => {
                let f1_name = f1.name();
                let f2_name = f2.name();
                if self.case_sensitive {
                    alphanumeric_sort::compare_str(&f1_name, &f2_name)
                } else {
                    let f1_name = f1_name.to_lowercase();
                    let f2_name = f2_name.to_lowercase();
                    alphanumeric_sort::compare_str(&f1_name, &f2_name)
                }
            }
            SortType::Mtime => mtime_sort(f1, f2),
        };

        if self.reverse {
            res = match res {
                cmp::Ordering::Less => cmp::Ordering::Greater,
                cmp::Ordering::Greater => cmp::Ordering::Less,
                s => s,
            };
        }
        res
    }
}

impl std::default::Default for SortOption {
    fn default() -> Self {
        SortOption {
            case_sensitive: false,
            reverse: false,
            sort_method: SortType::Natural,
        }
    }
}

fn mtime_sort(a: &TsuchitaEntry, b: &TsuchitaEntry) -> cmp::Ordering {
    match (a.get_type(), b.get_type()) {
        (EntryType::Message(a), EntryType::Message(b)) => {
            let a_mtime = a.timestamp();
            let b_mtime = b.timestamp();
            if a_mtime <= b_mtime {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            }
        }
        (EntryType::Message(_), _) => cmp::Ordering::Greater,
        _ => cmp::Ordering::Less,
    }
}
