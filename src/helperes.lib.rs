#![cfg_attr(not(debug_assertions),allow(non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]

pub mod alias;
pub mod helper;

use crate::alias 	::*;
use crate::helper	::*;

pub fn lib() -> i32 {0i32}
