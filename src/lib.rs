//! Synchronize code blocks between different files.
//!
//! It can replace `macros` in certain scenarios.
//! For example, when your code isn’t duplicated many times but the design is
//! relatively complex (e.g., involving generics), using `sync-code` instead
//! of a `macro` will give you much better readability and easier maintenance.
//! Features like Go-to-definition and intelligent auto-completion also work
//! properly without using `macros`.
//!
//! # Usage
//! Run command:
//! ```shell
//! cargo add --build sync-code
//! ```
//!
//! `build.rs`:
//! ```rust
//! fn main() {
//!     sync_code::Builder::new()
//!         .add("src/target1.rs", "src/source1.rs")
//!         .add("src/target2.rs", "src/source2.rs")
//!         .sync();
//! }
//!
//! ```
//!
//! `your_code.rs`:
//! ```rust
//! // $sync block_name
//!
//! fn code_you_want_to_sync() {
//! }
//!
//! // $sync end
//! ```

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
        println!("cargo:rerun-if-changed={}", dep_file.as_ref().display());
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
