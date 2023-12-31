use crate::common::Current;
use crate::constants::{Syscall, SyscallState};
use crate::pool::{CoroutinePool, CoroutinePoolImpl};
use crate::scheduler::listener::Listener;
use crate::scheduler::SchedulableCoroutine;
use std::sync::atomic::Ordering;

#[repr(C)]
#[derive(Debug, Default)]
pub(crate) struct CoroutineCreator {}

impl Listener for CoroutineCreator {
    fn on_suspend(&self, _: u64, _: &SchedulableCoroutine) {
        if let Some(pool) = CoroutinePoolImpl::current() {
            _ = pool.grow(true);
        }
    }

    fn on_syscall(&self, _: u64, _: &SchedulableCoroutine, _: Syscall, _: SyscallState) {
        if let Some(pool) = CoroutinePoolImpl::current() {
            _ = pool.grow(true);
        }
    }

    fn on_error(&self, _: u64, _: &SchedulableCoroutine, _: &str) {
        if let Some(pool) = CoroutinePoolImpl::current() {
            //worker协程异常退出，需要先回收再创建
            _ = pool.running.fetch_sub(1, Ordering::Release);
            _ = pool.grow(true);
        }
    }
}
