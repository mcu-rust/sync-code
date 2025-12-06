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
//! ```ignore
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

#![allow(clippy::needless_doctest_main)]
mod sync;

use std::path::Path;
use sync::Sync;

#[derive(Default)]
pub struct Builder {
    table: Vec<Sync>,
    file_list: Vec<String>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<P: AsRef<Path>>(mut self, file: P, dep_file: P) -> Self {
        self.file_list.push(file.as_ref().display().to_string());
        self.file_list.push(dep_file.as_ref().display().to_string());

        self.table.push(Sync::new(
            file.as_ref().to_path_buf(),
            dep_file.as_ref().to_path_buf(),
        ));
        self
    }

    pub fn sync(&mut self) {
        for sync in &self.table {
            sync.sync();
        }

        self.file_list.dedup();
        for file in &self.file_list {
            println!("cargo:rerun-if-changed={}", file);
        }
    }
}
