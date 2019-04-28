#![allow(dead_code)] // TODO(nlopes): remove at the end

extern crate arq;
extern crate clap;
extern crate rpassword;

pub mod cli;
pub mod computers;
pub mod error;
pub mod folders;
pub mod tree;
pub mod recovery;
mod utils;
