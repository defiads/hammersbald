//
// Copyright 2018 Tamas Blummer
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//!
//! # persistent store
//!
//! Implements persistent store

use error::BCDBError;
use logfile::LogFile;
use tablefile::TableFile;
use datafile::DataFile;
use linkfile::LinkFile;
use api::{BCDBFactory, BCDB};
use offset::Offset;
use page::Page;
use pagedfile::PagedFile;
use rolled::RolledFile;
use asyncfile::AsyncFile;
use cachedfile::CachedFile;

const TABLE_CHUNK_SIZE: u64 = 1024*1024*1024;
const DATA_CHUNK_SIZE: u64 = 1024*1024*1024;
const LOG_CHUNK_SIZE: u64 = 1024*1024*1024;

/// Implements persistent storage
pub struct Persistent {
    file: RolledFile
}

impl Persistent {
    /// create a new persistent DB
    pub fn new (file: RolledFile) -> Persistent {
        Persistent {file: file}
    }
}

impl BCDBFactory for Persistent {
    fn new_db (name: &str, cached_data_pages: usize) -> Result<BCDB, BCDBError> {
        let log = LogFile::new(
            Box::new(AsyncFile::new(
            Box::new(RolledFile::new(name.to_string(), "lg".to_string(), true, LOG_CHUNK_SIZE)?))?));
        let table = TableFile::new(Box::new(Persistent::new(
            RolledFile::new(name.to_string(), "tb".to_string(), false, TABLE_CHUNK_SIZE)?
        )))?;
        let link = LinkFile::new(Box::new(RolledFile::new(name.to_string(), "bl".to_string(), true, DATA_CHUNK_SIZE)?))?;
        let data = DataFile::new(
            Box::new(CachedFile::new(
            Box::new(AsyncFile::new(
            Box::new(RolledFile::new(
                name.to_string(), "bc".to_string(), true, DATA_CHUNK_SIZE)?))?), cached_data_pages)?))?;

        BCDB::new(log, table, data, link)
    }
}

impl PagedFile for Persistent {
    fn flush(&mut self) -> Result<(), BCDBError> {
        self.file.flush()
    }

    fn len(&self) -> Result<u64, BCDBError> {
        self.file.len()
    }

    fn truncate(&mut self, new_len: u64) -> Result<(), BCDBError> {
        self.file.truncate(new_len)
    }

    fn sync(&self) -> Result<(), BCDBError> {
        self.file.sync()
    }

    fn read_page(&self, offset: Offset) -> Result<Option<Page>, BCDBError> {
        self.file.read_page(offset)
    }

    fn append_page(&mut self, page: Page) -> Result<(), BCDBError> {
        self.file.append_page(page)
    }

    fn write_page(&mut self, offset: Offset, page: Page) -> Result<u64, BCDBError> {
        self.file.write_page(offset, page)
    }

    fn shutdown (&mut self) {
    }
}