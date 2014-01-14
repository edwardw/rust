#[crate_id = "std"];
#[crate_type = "rlib"];

#[feature(macro_rules, globs, managed_boxes)];

#[no_std];

/* The Prelude. */

pub mod prelude;

pub mod ptr;

/* Core language traits */

#[cfg(not(test))] pub mod kinds;
#[cfg(not(test))] pub mod ops;
#[cfg(not(test))] pub mod cmp;

/* Common traits */

pub mod iter;
pub mod clone;
pub mod container;
pub mod default;
pub mod any;

/* Common data structures */

pub mod option;

/* Runtime and platform support */

#[unstable]
pub mod libc;
pub mod cast;
#[deprecated]
pub mod util;
pub mod mem;

// Private APIs
#[unstable]
pub mod unstable;

#[unstable]
pub mod rt;

#[doc(hidden)]
mod std {
    pub use clone;
    pub use cmp;
    pub use kinds;
    pub use option;
    pub use rt;
    pub use unstable;
}
