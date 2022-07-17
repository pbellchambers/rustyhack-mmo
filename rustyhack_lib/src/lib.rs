#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

pub mod background_map;
pub mod consts;
pub mod ecs;
pub mod file_utils;
pub mod message_handler;

#[macro_use]
extern crate log;
extern crate simplelog;
