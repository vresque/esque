use core::{borrow::Borrow, ops::Deref, str::FromStr};

use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::{btree_map::Iter, BTreeMap},
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{debug, error, info};

static ENVIRONMENT: Once<Arc<RwLock<Environment>>> = Once::new();

#[derive(Debug, Clone)]
pub struct Environment {
    variables: BTreeMap<Box<str>, Box<str>>,
}

impl Environment {
    pub const fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
        }
    }

    /// # Get
    /// *Gets* the value of any environment variable
    /// If it does not exist, None is returned
    pub fn get(&self, name: &str) -> Option<&Box<str>> {
        Some(self.variables.get(name)?)
    }

    /// # Insert
    /// *Creates* a new environment variable.
    /// ## Notes
    /// If you wish to set an environment variable use set which will either set, or create the variable
    /// # Safety
    /// This value expects name and value to be valid UTF-8 strings
    pub fn insert(&mut self, name: &str, value: &str) {
        self.variables.insert(
            String::from_str(name).unwrap().into_boxed_str(),
            String::from_str(value).unwrap().into_boxed_str(),
        );
    }
    /// # Set
    /// *Sets* the environment variable.
    /// If it does not exist, it is inserted
    pub fn set(&mut self, name: &str, value: &str) {
        if self.exists(name) {
            // Safe at this point
            *(self.variables.get_mut(name).unwrap()) =
                String::from_str(value).unwrap().into_boxed_str();
        } else {
            self.insert(name, value)
        }
    }

    /// # Exists
    /// Returns whether an environment variable exists
    pub fn exists(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    pub fn iter(&self) -> Iter<'_, Box<str>, Box<str>> {
        self.variables.iter()
    }

    /// # Environ
    /// Returns a vector containing a pair of `VAR=VALUE` pairs.
    /// Can be turned into a pointer by using the environ_ptr method
    pub fn environ(self) -> Vec<Box<str>> {
        self.variables
            .iter()
            .map(|(name, val)| {
                let mut string = String::new();
                string.push_str(&name);
                string.push_str("=");
                string.push_str(&val);
                string.into_boxed_str()
            })
            .collect::<Vec<_>>()
    }

    pub fn environ_ptr(self) -> *mut *mut u8 {
        self.environ()
            .iter()
            .map(
                |x| x.as_ptr() as *mut u8, /* Cannot get mutable reference here */
            )
            .collect::<Vec<_>>()
            .as_mut_ptr()
    }
}

/// # Get Environment Variable
/// This function returns a value
/// The returned value is the value of the environment variable itself.
///     e.g. The value of SHELL is /bin/sh
pub fn getenv(name: &str) -> Option<Box<str>> {
    Some(environment_rw().read().get(name)?.to_owned())
}

pub fn setenv(name: &str, value: &str) {
    environment_mut().set(name, value);
}

pub fn environment_rw() -> &'static Arc<RwLock<Environment>> {
    ENVIRONMENT.call_once(|| Arc::new(RwLock::new(Environment::new())))
}

pub fn environment() -> RwLockReadGuard<'static, Environment> {
    ENVIRONMENT
        .call_once(|| Arc::new(RwLock::new(Environment::new())))
        .read()
}

pub fn environment_mut() -> RwLockWriteGuard<'static, Environment> {
    ENVIRONMENT
        .call_once(|| Arc::new(RwLock::new(Environment::new())))
        .write()
}
