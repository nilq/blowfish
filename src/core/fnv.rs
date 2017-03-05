extern crate fnv;

use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;

pub use self::fnv::FnvHasher;

pub type FnvMap<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;
pub type FnvSet<K>    = HashSet<K, BuildHasherDefault<FnvHasher>>;
