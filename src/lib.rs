#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub static VMAF_DEF_MODEL: &'static str = include_str!(concat!(env!("OUT_DIR"), "/vmaf/release/vmaf_v0.6.1.pkl"));
pub static VMAF_4K_MODEL: &'static str = include_str!(concat!(env!("OUT_DIR"), "/vmaf/release/vmaf_4k_v0.6.1.pkl"));

