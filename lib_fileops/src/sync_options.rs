/// Represent the options in the dir sync process
use std::path::PathBuf;

pub enum CompareStrategy {
    ModifiedTime,
    Size,
    SizeAndModifiedTime,
    Checksum,
}

pub enum DeleteStrategy {
    Remove,
    MoveToPath(PathBuf),
}

pub struct SyncOptions {
    /// Wether or not remove files that are present in destination dir but are
    /// not present in the origin dir
    delete_files_destination: bool,

    compare_strategy: CompareStrategy,
    exclude_patterns: Option<Vec<String>>,
    dry_run: bool,
    delete_strategy: DeleteStrategy,
    parallel_run: bool,
    include_hidden: bool,
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            delete_files_destination: false,
            compare_strategy: CompareStrategy::ModifiedTime,
            exclude_patterns: None,
            dry_run: false,
            delete_strategy: DeleteStrategy::Remove,
            parallel_run: false,
            include_hidden: false,
        }
    }
}

/// Builder pattern to improve ergonomics when using this type

pub struct SyncOptionsBuilder {
    options: SyncOptions,
}

impl SyncOptionsBuilder {
    pub fn new() -> Self {
        Self {
            options: SyncOptions::default(),
        }
    }
}

impl SyncOptionsBuilder {
    pub fn delete_files_destination(mut self, value: bool) -> Self {
        self.options.delete_files_destination = value;
        self
    }

    pub fn compare_strategy(mut self, strategy: CompareStrategy) -> Self {
        self.options.compare_strategy = strategy;
        self
    }

    pub fn exclude_patterns(mut self, patterns: Vec<String>) -> Self {
        self.options.exclude_patterns = Some(patterns);
        self
    }

    pub fn add_exclude_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.options.exclude_patterns = match self.options.exclude_patterns {
            Some(mut values) => {
                values.push(pattern.into());
                Some(values)
            }
            None => Some(vec![pattern.into()]),
        };
        self
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.options.dry_run = value;
        self
    }

    pub fn delete_strategy(mut self, strategy: DeleteStrategy) -> Self {
        self.options.delete_strategy = strategy;
        self
    }

    pub fn parallel_run(mut self, value: bool) -> Self {
        self.options.parallel_run = value;
        self
    }

    pub fn include_hidden(mut self, value: bool) -> Self {
        self.options.include_hidden = value;
        self
    }

    pub fn build(self) -> SyncOptions {
        self.options
    }
}

impl SyncOptions {
    pub fn builder() -> SyncOptionsBuilder {
        SyncOptionsBuilder::new()
    }
}
