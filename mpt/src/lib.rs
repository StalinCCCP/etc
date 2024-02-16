mod utils;
use utils::hashing;
mod defs;
use defs::{Hash,Path,Data};
mod node;
pub mod mpt;
pub use mpt::MPT;