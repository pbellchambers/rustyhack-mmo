#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![warn(clippy::pedantic)]
#![allow(clippy::unreadable_literal)]

pub mod background_map;
pub mod consts;
pub mod ecs;
pub mod file_utils;
pub mod math_utils;
pub mod message_handler;

#[macro_use]
extern crate log;
extern crate simplelog;
