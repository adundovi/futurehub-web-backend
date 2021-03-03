#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod db;
pub mod cli;
pub mod rest;
pub mod tools;
