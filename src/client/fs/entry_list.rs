use std::slice::{Iter, IterMut};

use super::TsuchitaMessage;

#[derive(Clone, Debug)]
pub enum EntryType {
    Source,
    Message(TsuchitaMessage),
}

#[derive(Clone, Debug)]
pub struct TsuchitaEntry {
    _name: String,
    _type: EntryType,
}

impl TsuchitaEntry {
    pub fn new(_name: String, _type: EntryType) -> Self {
        Self { _name, _type }
    }

    pub fn name(&self) -> &str {
        self._name.as_str()
    }

    pub fn get_type(&self) -> &EntryType {
        &self._type
    }
}

#[derive(Clone, Debug)]
pub struct TsuchitaList {
    pub index: usize,
    name: String,
    contents: Vec<TsuchitaEntry>,
    content_outdated: bool,
}

impl TsuchitaList {
    pub fn new(name: String, contents: Vec<TsuchitaEntry>) -> Self {
        Self {
            name,
            index: 0,
            contents,
            content_outdated: false,
        }
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn iter(&self) -> Iter<TsuchitaEntry> {
        self.contents.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<TsuchitaEntry> {
        self.contents.iter_mut()
    }

    pub fn needs_update(&self) -> bool {
        self.content_outdated
    }

    pub fn curr_entry_ref(&self) -> Option<&TsuchitaEntry> {
        self._curr_entry_ref(self.index)
    }

    fn _curr_entry_ref(&self, index: usize) -> Option<&TsuchitaEntry> {
        if index < self.contents.len() {
            Some(&self.contents[index])
        } else {
            None
        }
    }
}
