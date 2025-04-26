#![feature(hash_set_entry)]

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ptr::addr_eq;
use std::sync::{LazyLock, PoisonError, RwLock};

pub trait Internable: 'static + PartialEq + Eq + Hash {
    fn intern(&self) -> Interned<Self>;

    fn leak(&self) -> &'static Self;
}

#[derive(Debug)]
pub struct Interned<T: Internable + ?Sized> {
    instance: &'static T,
}

impl<T: Internable + ?Sized> Copy for Interned<T> {}
impl<T: Internable + ?Sized> Clone for Interned<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            instance: self.instance,
        }
    }
}

impl<T: Internable + ?Sized> PartialEq for Interned<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        addr_eq(self.instance, other.instance)
    }
}

impl<T: Internable + ?Sized> Eq for Interned<T> {}

impl<T: Internable + ?Sized> Hash for Interned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.instance as *const T).hash(state)
    }
}

pub struct Interner<T: Internable + ?Sized> {
    map: RwLock<HashSet<&'static T>>,
}

impl<T: Internable + ?Sized> Interner<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashSet::new()),
        }
    }

    #[inline]
    pub fn intern(&self, value: &T) -> Interned<T> {
        if let Some(&instance) = self
            .map
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .get(value)
        {
            Interned { instance }
        } else {
            let instance = *self
                .map
                .write()
                .unwrap_or_else(PoisonError::into_inner)
                .get_or_insert_with(value, Internable::leak);
            Interned { instance }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Label(pub &'static str);
impl Internable for Label {
    #[inline]
    fn intern(&self) -> Interned<Self> {
        static INTERNER: LazyLock<Interner<Label>> = LazyLock::new(Interner::new);
        INTERNER.intern(self)
    }

    #[inline]
    fn leak(&self) -> &'static Self {
        Box::leak(Box::new(Self(self.0)))
    }
}
