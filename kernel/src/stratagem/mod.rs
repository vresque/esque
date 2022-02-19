use core::sync::atomic::{AtomicU64, AtomicUsize};

use alloc::{boxed::Box, collections::BTreeMap, sync::Arc};

use self::traits::Stratagem;

pub mod traits;

pub type ArcStratagem = Arc<dyn Stratagem + Send + Sync>;

num_backed::num_backed!(
    Namespace backed by u64;
    atomic: AtomicNamespace backed by AtomicU64
);

num_backed::num_backed!(
    StratagemID backed by u64;
    atomic: AtomicStratagemID backed by AtomicU64
);

num_backed::num_backed!(
    FileHandle backed by u64;
    atomic: AtomicFileHandle backed by AtomicU64
);

pub struct StratagemTracker {
    namespaces: BTreeMap<Namespace, BTreeMap<Box<str>, StratagemID>>,
    last_id: u64,
    last_namespace: u64,
    id_map: BTreeMap<StratagemID, ArcStratagem>,
}

impl StratagemTracker {
    pub fn new() -> Self {
        let mut tracker = Self {
            namespaces: BTreeMap::new(),
            last_id: 0,        /* 0 is reserved for the genesis namespace */
            last_namespace: 0, /* 0 is reserved for the genesis namespace */
            id_map: BTreeMap::new(),
        };
        tracker.create_genesis_namespace();
        tracker.create_root_namespace();
        tracker
    }

    pub fn create_genesis_namespace(&mut self) {}
    pub fn create_root_namespace(&mut self) {}
}
