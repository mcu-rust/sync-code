mod sync;

use std::path::Path;
use sync::Sync;

pub struct Builder {
    table: Vec<Sync>,
}

impl Builder {
    pub fn new() -> Self {
        Builder { table: Vec::new() }
    }

    pub fn add<P: AsRef<Path>>(mut self, file: P, dep_file: P) -> Self {
        // println!("cargo:rerun-if-changed={}", dep_file.as_ref().display());
        self.table.push(Sync::new(
            file.as_ref().to_path_buf(),
            dep_file.as_ref().to_path_buf(),
        ));
        self
    }

    pub fn sync(&mut self) {
        println!("Start syncing code ... {}", self.table.len());
        for sync in &self.table {
            sync.sync();
        }
    }
}
