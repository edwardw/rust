#[crate_id = "std"];
#[crate_type = "rlib"];

#[feature(macro_rules, globs, managed_boxes)];

#[no_std];

/* The Prelude. */

pub mod prelude;

/* Primitive types */

#[path = "num/float_macros.rs"] mod float_macros;
#[path = "num/int_macros.rs"]   mod int_macros;
#[path = "num/uint_macros.rs"]  mod uint_macros;

#[path = "num/int.rs"]  pub mod int;
#[path = "num/i8.rs"]   pub mod i8;
#[path = "num/i16.rs"]  pub mod i16;
#[path = "num/i32.rs"]  pub mod i32;
#[path = "num/i64.rs"]  pub mod i64;

#[path = "num/uint.rs"] pub mod uint;
#[path = "num/u8.rs"]   pub mod u8;
#[path = "num/u16.rs"]  pub mod u16;
#[path = "num/u32.rs"]  pub mod u32;
#[path = "num/u64.rs"]  pub mod u64;

#[path = "num/f32.rs"]   pub mod f32;
#[path = "num/f64.rs"]   pub mod f64;

pub mod char;
pub mod tuple;

pub mod vec;
pub mod str;

pub mod ptr;
pub mod owned;
pub mod borrow;

/* Core language traits */

#[cfg(not(test))] pub mod kinds;
#[cfg(not(test))] pub mod ops;
#[cfg(not(test))] pub mod cmp;

/* Common traits */

pub mod from_str;
pub mod num;
pub mod iter;
pub mod to_str;
pub mod to_bytes;
pub mod clone;
pub mod container;
pub mod default;
pub mod any;

/* Common data structures */

pub mod option;

/* Tasks and communication */

pub mod local_data;

/* Runtime and platform support */

#[unstable]
pub mod libc;
pub mod cast;
pub mod io;
pub mod fmt;
#[deprecated]
pub mod condition;
pub mod logging;
pub mod util;
pub mod mem;

/* Unsupported interfaces */

#[unstable]
pub mod repr;
#[unstable]
pub mod reflect;

// Private APIs
#[unstable]
pub mod unstable;

/* For internal use, not exported */

mod unicode;
#[path = "num/cmath.rs"]
mod cmath;

#[unstable]
pub mod rt;

#[doc(hidden)]
mod std {
    pub use clone;
    pub use cmp;
    pub use condition;
    pub use fmt;
    pub use io;
    pub use kinds;
    pub use local_data;
    pub use logging;
    pub use option;
    pub use rt;
    pub use str;
    pub use to_bytes;
    pub use to_str;
    pub use unstable;
}
