#[crate_id = "std"];
#[crate_type = "rlib"];

#[feature(macro_rules, globs, managed_boxes)];

#[no_std];

/* The Prelude. */

pub mod prelude;

/* Core language traits */

#[cfg(not(test))] pub mod kinds;
#[cfg(not(test))] pub mod ops;
#[cfg(not(test))] pub mod cmp;

/* Common traits */

pub mod clone;
pub mod default;

#[doc(hidden)]
mod std {
    pub use clone;
    pub use cmp;
    pub use kinds;
}
