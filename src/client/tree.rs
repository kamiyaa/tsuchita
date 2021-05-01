use std::collections::HashMap;

use crate::fs::{EntryType, TsuchitaEntry, TsuchitaList, TsuchitaMessage};
use crate::util::display::DisplayOption;
use crate::util::format;

pub type DbusTree = HashMap<String, TsuchitaList>;

pub trait DbusTreeTrait {
    fn fetch_sources(
        &mut self,
        url: &str,
        display_options: &DisplayOption,
    ) -> Result<(), ureq::Error>;
    fn fetch_messages(
        &mut self,
        url: &str,
        source: &str,
        display_options: &DisplayOption,
    ) -> Result<(), ureq::Error>;
    fn get_sources(&self) -> Option<&TsuchitaList>;
}

impl DbusTreeTrait for DbusTree {
    fn fetch_sources(
        &mut self,
        url: &str,
        display_options: &DisplayOption,
    ) -> Result<(), ureq::Error> {
        let url = format!("http://{}/sources/", url);
        let mut json_res: Vec<String> = ureq::get(url.as_str()).call()?.into_json()?;
        json_res.sort();

        let mut contents: Vec<TsuchitaEntry> = json_res
            .drain(..)
            .map(|s| TsuchitaEntry::new(s, EntryType::Source))
            .collect();

        let name = "/".to_string();
        let list = TsuchitaList::new(name.clone(), contents);
        self.insert(name, list);
        Ok(())
    }

    fn fetch_messages(
        &mut self,
        url: &str,
        source: &str,
        display_options: &DisplayOption,
    ) -> Result<(), ureq::Error> {
        let url = format!("http://{}/source/{}/messages/", url, source);
        let mut json_res: Vec<TsuchitaMessage> = ureq::get(url.as_str()).call()?.into_json()?;

        let mut contents: Vec<TsuchitaEntry> = json_res
            .drain(..)
            .map(|m| {
                let timestamp = format::time_to_local(m.timestamp(), display_options.date_format());
                let name = format!("{} {}", timestamp, m.title());
                TsuchitaEntry::new(name, EntryType::Message(m))
            })
            .collect();

        let sort_options = display_options.sort_options_ref();
        contents.sort_by(|f1, f2| sort_options.compare(f1, f2));

        let name = source.to_string();
        let list = TsuchitaList::new(name.clone(), contents);
        self.insert(name, list);

        Ok(())
    }

    fn get_sources(&self) -> Option<&TsuchitaList> {
        self.get("/")
    }
}
