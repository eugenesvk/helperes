#![cfg_attr(not(debug_assertions),allow(non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]

pub mod alias;
pub mod helper;
#[cfg(feature="stros")]
pub mod stros;

use crate::alias 	::*;
use crate::helper	::*;
#[cfg(feature="stros")]
use crate::stros	::*;


pub fn lib() -> i32 {0i32}
