#![feature(trait_alias)]

extern crate config;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod db;
pub mod settings;
pub mod startup;
pub mod web;
pub mod state;
