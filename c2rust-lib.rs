#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(register_tool)]
#![register_tool(c2rust)]


extern crate libc;
pub mod src {
pub mod helpers;
pub mod light;
pub mod r#impl {
pub mod razer;
pub mod sysfs;
pub mod util;
} // mod r#impl
} // mod src
