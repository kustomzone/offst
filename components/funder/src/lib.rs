#![crate_type = "lib"] 
#![feature(nll)]
#![feature(try_from)]
#![feature(generators)]
#![feature(never_type)]
#![cfg_attr(not(feature = "cargo-clippy"), allow(unknown_lints))]

// TODO: Remove this later:
#![allow(dead_code, unused)]

extern crate byteorder;
extern crate bytes;
// extern crate futures;
extern crate futures_await as futures;
extern crate futures_cpupool;
#[macro_use]
extern crate log;
extern crate rand;
// extern crate rusqlite;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_codec;

extern crate num_traits;
extern crate num_bigint;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate base64;

extern crate atomicwrites;

extern crate im;

extern crate cswitch_utils as utils;
extern crate cswitch_crypto as crypto;
extern crate cswitch_identity as identity;

pub use self::macros::TryFromBytesError;

#[macro_use]
mod macros;
pub mod messages;
mod liveness;
mod ephemeral;
mod credit_calc;
mod freeze_guard;
mod signature_buff; 
mod friend;
mod state;
mod types;
mod token_channel;
mod handler;
mod report;
mod database;
mod funder;