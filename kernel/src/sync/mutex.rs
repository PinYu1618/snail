// ! Code Ref: https://github.com/rcore-os/rcore/kernel/src/sync/mutex.rs

use alloc::{collections::VecDeque, sync::Arc};

use crate::{sync::UPSafeCell, task_::TaskCtrlBlock};
use crate::sync::Condvar;
use core::ops::{DerefMut, Deref};
use core::sync::atomic::{AtomicBool, AtomicU8};
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

pub struct MutexSpin {
    pub locked: UPSafeCell<bool>,
}

pub struct MutexBlocking {
    pub inner: UPSafeCell<MutexBlockingInner>,
}

pub struct MutexBlockingInner {
    pub locked: bool,
    pub wait_queue: VecDeque<Arc<TaskCtrlBlock>>,
}

pub type SpinLock<T> = Mutex<T, Spin>;
pub type SpinNoIrqLock<T> = Mutex<T, SpinNoIrq>;
pub type SleepLock<T> = Mutex<T, Condvar>;

pub struct Mutex<T: ?Sized, S: MutexSupport> {
    lock: AtomicBool,
    support: MaybeUninit<S>,
    support_init: AtomicU8,
    user: UnsafeCell<(usize, usize)>,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: ?Sized + 'a, S: MutexSupport + 'a> {
    pub(super) mutex: &'a Mutex<T, S>,
    support_guard: S::Guard,
}

// Same unsafe impls as `std::sync::Mutex`
unsafe impl<T: ?Sized + Send, S: MutexSupport> Sync for Mutex<T, S> {}
unsafe impl<T: ?Sized + Send, S: MutexSupport> Send for Mutex<T, S> {}

impl<T, S: MutexSupport> Mutex<T, S> {
    pub const fn new(user_data: T) -> Mutex<T, S> {
        Mutex {
            lock: AtomicBool::new(false),
            support: MaybeUninit::uninit(),
            support_init: AtomicU8::new(0),
            user: UnsafeCell::new((0, 0)),
            data: UnsafeCell::new(user_data),
        }
    }

    /// Consumes this mutex, return underlying data
    pub fn into_inner(self) -> T {
        let Mutex {data, ..} = self;
        data.into_inner()
    }
}

impl<T: ?Sized, S: MutexSupport> Mutex<T, S> {
    pub fn lock(&self) -> MutexGuard<T, S> {
        todo!()
    }

    pub fn busy_lock(&self) -> MutexGuard<T, S> {
        todo!()
    }

    pub fn ensure_support(&self) {
        let _init = self.support_init.load(core::sync::atomic::Ordering::Relaxed);
        todo!()
    }

    fn obtain_lock(&self) {
        todo!()
    }
}

impl<T: ?Sized + Default, S: MutexSupport> Default for Mutex<T, S> {
    fn default() -> Self {
        Mutex::new(Default::default())
    }
}

impl<'a, T: ?Sized, S: MutexSupport> Deref for MutexGuard<'a, T, S> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T: ?Sized, S: MutexSupport> DerefMut for MutexGuard<'a, T, S> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T: ?Sized, S: MutexSupport> Drop for MutexGuard<'a, T, S> {
    fn drop(&mut self) {
        todo!()
    }
}

/// Low level support for mutex
pub trait MutexSupport {
    type Guard;
    fn new() -> Self;
    /// Called when failing to acquire the lock
    fn cpu_relax(&self);
    /// Called before lock() & try_lock()
    fn before_lock() -> Self::Guard;
    /// Called when Guard dropping
    fn after_unlock(&self);
}

/// Spin lock
#[derive(Debug)]
pub struct Spin;

impl MutexSupport for Spin {
    type Guard = ();
    
    fn new() -> Self {
        Self
    }
    fn cpu_relax(&self) {
        core::hint::spin_loop();
    }
    fn before_lock() -> Self::Guard {}
    fn after_unlock(&self) {}
}

/// Spin & no interrupt lock
#[derive(Debug)]
pub struct SpinNoIrq;

/// Contains RFLAGS before disable interrupt, will auto restore it when dropping
pub struct FlagsGuard;

impl Drop for FlagsGuard {
    fn drop(&mut self) {
        todo!()
    }
}

impl FlagsGuard {
    pub fn no_irq_region() -> Self {
        todo!()
    }
}

impl MutexSupport for SpinNoIrq {
    type Guard = FlagsGuard;

    fn new() -> Self {
        Self
    }
    fn cpu_relax(&self) {
        core::hint::spin_loop();
    }
    fn before_lock() -> Self::Guard {
        todo!()
    }
    fn after_unlock(&self) {}
}

impl MutexSupport for Condvar {
    type Guard = ();

    fn new() -> Self {
        todo!()
    }
    fn cpu_relax(&self) {
        todo!()
    }
    fn before_lock() -> Self::Guard {}
    fn after_unlock(&self) {
        todo!()
    }
}
