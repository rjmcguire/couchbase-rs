//! This crate represents the official, yet under heavy development, Rust SDK for Couchbase.
//!
//! It is based on the `couchbase_sys`-crate, which in turn consists of Rust bindings to the
//! [libcouchbase](https://github.com/couchbase/libcouchbase) c-library.
//!
//! # Examples
//!
//! Reading and writing a `Document` is simple:
//!
//! ```rust,no_run
//! extern crate couchbase;
//! extern crate futures;
//!
//! use couchbase::{Document, Cluster};
//! use futures::Future;
//!
//! /// A very simple example which connects to the `default` bucket and writes and loads
//! /// a document.
//! fn main() {
//!     // Initialize the Cluster
//!     let cluster = Cluster::new("localhost").expect("Could not initialize Cluster");
//!
//!     // Open the travel-sample bucket
//!     let bucket = cluster.open_bucket("default", "").expect("Could not open Bucket");
//!
//!     // Create a document and store it in the bucket
//!     let document = Document::from_str("hello", "{\"world\":true}");
//!     println!("Wrote Document {:?}",
//!              bucket.upsert(document)
//!                  .wait()
//!                  .unwrap());
//!
//!     // Load the previously written document and print it out
//!     match bucket.get("hello").wait().expect("Could not load Document") {
//!         Some(d) => println!("Found Document {:?}", d),
//!         None => println!("Document not found!"),
//!     }
//!
//! }
//! ```
//!
//! For now, more examples can be found under `examples`. Note that for all the `serde`-based
//! examples you need to at least have Rust 1.15.0 installed.
//!
extern crate couchbase_sys;
extern crate futures;
extern crate parking_lot;

pub mod bucket;
pub mod cluster;
pub mod document;
pub mod sync;

pub use document::Document;
pub use bucket::Bucket;
pub use cluster::Cluster;
pub use sync::CouchbaseFuture;

use couchbase_sys::*;

/// Generic error type, might change in the future.
pub type CouchbaseError = lcb_error_t;
