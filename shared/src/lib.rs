// protocols

use std::env;

pub mod constants {
    include!(concat!(env!("OUT_DIR"), "/constants.rs"));
}

pub mod protocol;

