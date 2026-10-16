//! # landmine
//! 
//! **Boom.**
//! 
//! ```no_run
//! use landmine::Landmine;
//! 
//! fn landmine() {
//!     let landmine = Landmine::Safe(());
//!     drop(landmine);
//! 
//!     let number = Landmine::Safe(5);
//!     let inner = number.unwrap();
//! 
//!     let landmine: Landmine<i32> = Landmine::Armed;
//!     let defused = landmine.defuse();
//! }
//! ```
mod operations;
mod traits;

/// A wrapper type that forces correct handling.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Landmine<T> {
    Armed,
    Safe(T)
}