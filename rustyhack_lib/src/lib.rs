#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![warn(clippy::pedantic)]
#![allow(clippy::unreadable_literal)]

pub mod background_map;
pub mod consts;
pub mod ecs;
pub mod network;
pub mod utils;

#[macro_use]
extern crate log;
extern crate simplelog;
