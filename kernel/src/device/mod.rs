use core::sync::atomic::{AtomicU64, AtomicUsize};

use crate::alloc::string::ToString;
use alloc::{boxed::Box, collections::BTreeMap, sync::Arc};

use crate::error::Error;
use crate::error::Result;

use self::{kernel::Kernel, traits::Device};
pub mod kernel;
pub mod traits;

pub type ArcDevice = Arc<dyn Device + Send + Sync>;

num_backed::num_backed!(
    Namespace backed by u64;
    atomic: AtomicNamespace backed by AtomicU64
);

num_backed::num_backed!(
    DeviceID backed by u64;
    atomic: AtomicDeviceID backed by AtomicU64
);

num_backed::num_backed!(
    FileHandle backed by u64;
    atomic: AtomicFileHandle backed by AtomicU64
);

pub struct DeviceTracker {
    namespaces: BTreeMap<
        Namespace,
        BTreeMap<(Box<str> /* Real Path */, Box<str> /* Fake Path */), DeviceID>,
    >,
    last_id: u64,
    last_namespace: u64,
    devices: BTreeMap<DeviceID, ArcDevice>,
}

impl DeviceTracker {
    pub fn new() -> Self {
        let mut tracker = Self {
            namespaces: BTreeMap::new(),
            last_id: 0,        /* 0 is reserved for the genesis namespace */
            last_namespace: 0, /* 0 is reserved for the genesis namespace */
            devices: BTreeMap::new(),
        };
        tracker.create_genesis_namespace();
        tracker.create_root_namespace();
        tracker
    }

    pub fn create_genesis_namespace(&mut self) {
        let genesis = Namespace(0);
        self.push_namespace(genesis);
        self.insert_device_at(genesis, "kernel", "/dev/kernel", |id| {
            Arc::new(Kernel::new(id))
        });
    }

    pub fn create_root_namespace(&mut self) {}

    pub fn create_namespace(&mut self) -> Namespace {
        let namespace = Namespace(self.last_namespace + 1);
        self.push_namespace(namespace);
        namespace
    }

    pub fn push_namespace(&mut self, namespace: Namespace) {
        self.namespaces.insert(namespace, BTreeMap::new());
        self.last_namespace = namespace.0;
    }

    pub fn insert_device_at(
        &mut self,
        namespace: Namespace,
        real_name: &str,
        fake_name: &str,
        func: fn(DeviceID) -> ArcDevice,
    ) -> Result<DeviceID> {
        let fake = fake_name.to_string().into_boxed_str();
        let real = real_name.to_string().into_boxed_str();
        // Does it already exist?
        if let Some(name) = self.namespaces.get(&namespace) {
            if name.contains_key(&(real, fake)) {
                return Err(Error::AlreadyExists);
            }
        }

        let projected_device_id = DeviceID(self.last_id + 1);
        let device = func(projected_device_id);
        self.devices.insert(projected_device_id, device).unwrap();
        return match self.namespaces.get_mut(&namespace) {
            Some(x) => {
                x.insert((real, fake), projected_device_id);
                Ok(projected_device_id)
            }
            None => Err(Error::NoSuchDevice),
        };
    }
}
