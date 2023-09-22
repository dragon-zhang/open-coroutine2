use crate::coroutine::constants::{Syscall, SyscallState};
use crate::coroutine::Current;
use crate::pool::{CoroutinePool, CoroutinePoolImpl};
use crate::scheduler::{Listener, SchedulableCoroutine};
use std::sync::atomic::Ordering;

#[derive(Debug)]
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
            _ = pool.running.fetch_add(1, Ordering::Release);
            _ = pool.grow(true);
        }
    }
}
