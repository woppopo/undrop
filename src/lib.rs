#![no_std]

use core::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Debug)]
pub struct Undroppable<T>(T);

impl<T> Undroppable<T> {
    const PANIC: () = panic!("Undroppable!");

    pub const fn new(value: T) -> Self {
        Self(value)
    }

    pub const fn forget(this: Self) {
        core::mem::forget(this);
    }

    pub fn into_inner(mut this: Self) -> T {
        let inner = unsafe { Self::take(&mut this) };
        Self::forget(this);
        inner
    }

    pub unsafe fn take(this: &mut Self) -> T {
        unsafe { core::ptr::read(&this.0) }
    }
}

impl<T> Deref for Undroppable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Undroppable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for Undroppable<T> {
    fn drop(&mut self) {
        Self::PANIC
    }
}
