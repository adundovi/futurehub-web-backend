#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod db;
pub mod cli;
pub mod consts;
pub mod rest;
pub mod services;
pub mod tools;

pub fn active_config() -> rocket::Config {
    let c = rocket::config::RocketConfig::read().unwrap();
    c.active().clone()
}

