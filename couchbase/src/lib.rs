#[macro_use]
extern crate log;
extern crate couchbase_sys;
extern crate futures;

pub mod bucket;

pub use bucket::Bucket;

use couchbase_sys::*;
use futures::{Async, Future, Poll};
use futures::sync::oneshot::Receiver;
use std::panic;

pub type CouchbaseError = lcb_error_t;

#[derive(Debug)]
pub struct Document {
    id: String,
    cas: u64,
    content: String,
    expiry: i32,
}

impl Document {
    pub fn cas(&self) -> u64 {
        self.cas
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn expiry(&self) -> i32 {
        self.expiry
    }
}


pub struct CouchbaseFuture<T, E> {
    inner: Receiver<Result<T, E>>,
}

impl<T: Send + 'static, E: Send + 'static> Future for CouchbaseFuture<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<T, E> {
        match self.inner.poll().expect("shouldn't be canceled") {
            Async::NotReady => Ok(Async::NotReady),
            Async::Ready(Err(e)) => panic::resume_unwind(Box::new(e)),
            Async::Ready(Ok(e)) => Ok(Async::Ready(e)),
        }
    }
}
