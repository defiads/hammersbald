//
// Copyright 2018-2019 Tamas Blummer
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
//! # Hammersbald Blockchain store
//!
//! A very fast persistent blockchain store
//!

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

#[cfg(feature="bitcoin_support")]extern crate bitcoin;
#[cfg(feature="bitcoin_support")]extern crate serde;
#[cfg(feature="bitcoin_support")]extern crate serde_cbor;
extern crate bitcoin_hashes;
extern crate rand;
extern crate byteorder;
extern crate lru_cache;

mod page;
mod pagedfile;
mod logfile;
mod tablefile;
mod cachedfile;
mod singlefile;
mod rolledfile;
mod asyncfile;
mod memtable;
mod format;
mod datafile;
mod pref;
mod transient;
mod persistent;
mod error;
mod stats;
mod api;

pub use pref::PRef;
pub use error::Error;
pub use api::{
    HammersbaldAPI,
    HammersbaldDataWriter,
    HammersbaldDataReader,
    HammersbaldIterator,
    persistent,
    transient
};

#[cfg(feature="bitcoin_support")]
mod bitcoin_adaptor;

#[cfg(feature="bitcoin_support")]
pub use bitcoin_adaptor::BitcoinAdaptor;

#[cfg(feature="cbor")]
mod cbor_adaptor;

#[cfg(feature="cbor")]
pub use cbor_adaptor::CBORAdaptor;