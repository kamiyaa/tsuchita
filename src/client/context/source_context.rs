pub struct SourceContext {
    pub index: usize,
    sources: Vec<String>,
}

impl SourceContext {
    pub fn new(sources: Vec<String>, index: usize) -> Self {
        Self { sources, index }
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }
    pub fn len(&self) -> usize {
        self.sources.len()
    }
    pub fn sources_ref(&self) -> &[String] {
        self.sources.as_slice()
    }

    pub fn curr_source(&self) -> Option<&str> {
        if self.sources.is_empty() {
            None
        } else {
            Some(self.sources[self.index].as_str())
        }
    }
}

impl std::default::Default for SourceContext {
    fn default() -> Self {
        Self {
            index: 0,
            sources: Vec::new(),
        }
    }
}
