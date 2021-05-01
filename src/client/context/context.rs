use std::sync::mpsc;

use crate::config::AppConfig;
use crate::fs::TsuchitaList;
use crate::tree::DbusTree;
use crate::util::event::{Events, TsuchitaEvent};
use crate::util::search::SearchPattern;

pub struct AppContext {
    pub exit: bool,
    config: AppConfig,
    events: Events,

    tree: DbusTree,
    path: String,

    search_state: Option<SearchPattern>,
}

impl AppContext {
    pub fn new(config: AppConfig) -> Self {
        Self {
            exit: false,
            config,
            events: Events::default(),

            tree: DbusTree::default(),
            path: "/".to_string(),

            search_state: None,
        }
    }
    // event related
    pub fn poll_event(&self) -> Result<TsuchitaEvent, mpsc::RecvError> {
        self.events.next()
    }
    pub fn get_event_tx(&self) -> mpsc::Sender<TsuchitaEvent> {
        self.events.event_tx.clone()
    }
    pub fn flush_event(&self) {
        self.events.flush();
    }

    pub fn config_ref(&self) -> &AppConfig {
        &self.config
    }
    pub fn config_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }

    pub fn set_search_state(&mut self, pattern: SearchPattern) {
        self.search_state = Some(pattern);
    }
    pub fn get_search_state(&self) -> Option<&SearchPattern> {
        self.search_state.as_ref()
    }

    pub fn path_ref(&self) -> &str {
        self.path.as_str()
    }
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn curr_list_ref(&self) -> Option<&TsuchitaList> {
        self.tree_ref().get(self.path.as_str())
    }
    pub fn curr_list_mut(&mut self) -> Option<&mut TsuchitaList> {
        let s = self.path.clone();
        self.tree_mut().get_mut(s.as_str())
    }

    pub fn tree_ref(&self) -> &DbusTree {
        &self.tree
    }
    pub fn tree_mut(&mut self) -> &mut DbusTree {
        &mut self.tree
    }
}
