#[cfg(unix)]
use libc::{c_int, c_void, iovec, msghdr, off_t, size_t, sockaddr, socklen_t, ssize_t};

pub mod raw;

pub mod nio;

pub mod io_uring;

pub mod facade;

#[cfg(unix)]
pub trait UnixNetSyscall {
    /// socket

    fn socket(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, c_int, c_int) -> c_int>,
        domain: c_int,
        ty: c_int,
        protocol: c_int,
    ) -> c_int {
        if let Some(f) = fn_ptr {
            (f)(domain, ty, protocol)
        } else {
            unsafe { libc::socket(domain, ty, protocol) }
        }
    }

    fn accept(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *mut sockaddr, *mut socklen_t) -> c_int>,
        socket: c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> c_int {
        if let Some(f) = fn_ptr {
            (f)(socket, address, address_len)
        } else {
            unsafe { libc::accept(socket, address, address_len) }
        }
    }

    fn connect(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *const sockaddr, socklen_t) -> c_int>,
        socket: c_int,
        address: *const sockaddr,
        len: socklen_t,
    ) -> c_int {
        if let Some(f) = fn_ptr {
            (f)(socket, address, len)
        } else {
            unsafe { libc::connect(socket, address, len) }
        }
    }

    fn shutdown(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, c_int) -> c_int>,
        socket: c_int,
        how: c_int,
    ) -> c_int {
        if let Some(f) = fn_ptr {
            (f)(socket, how)
        } else {
            unsafe { libc::shutdown(socket, how) }
        }
    }

    fn close(&self, fn_ptr: Option<&extern "C" fn(c_int) -> c_int>, fd: c_int) -> c_int {
        if let Some(f) = fn_ptr {
            (f)(fd)
        } else {
            unsafe { libc::close(fd) }
        }
    }

    /// read

    fn recv(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *mut c_void, size_t, c_int) -> ssize_t>,
        socket: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(socket, buf, len, flags)
        } else {
            unsafe { libc::send(socket, buf, len, flags) }
        }
    }

    fn read(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *mut c_void, size_t) -> ssize_t>,
        fd: c_int,
        buf: *mut c_void,
        count: size_t,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(fd, buf, count)
        } else {
            unsafe { libc::read(fd, buf, count) }
        }
    }

    fn pread(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *mut c_void, size_t, off_t) -> ssize_t>,
        fd: c_int,
        buf: *mut c_void,
        count: size_t,
        offset: off_t,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(fd, buf, count, offset)
        } else {
            unsafe { libc::pread(fd, buf, count, offset) }
        }
    }

    fn readv(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *const iovec, c_int) -> ssize_t>,
        fd: c_int,
        iov: *const iovec,
        iovcnt: c_int,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(fd, iov, iovcnt)
        } else {
            unsafe { libc::readv(fd, iov, iovcnt) }
        }
    }

    fn preadv(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *const iovec, c_int, off_t) -> ssize_t>,
        fd: c_int,
        iov: *const iovec,
        iovcnt: c_int,
        offset: off_t,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(fd, iov, iovcnt, offset)
        } else {
            unsafe { libc::preadv(fd, iov, iovcnt, offset) }
        }
    }

    fn recvmsg(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *mut msghdr, c_int) -> ssize_t>,
        fd: c_int,
        msg: *mut msghdr,
        flags: c_int,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(fd, msg, flags)
        } else {
            unsafe { libc::recvmsg(fd, msg, flags) }
        }
    }

    /// write

    fn send(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *const c_void, size_t, c_int) -> ssize_t>,
        socket: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(socket, buf, len, flags)
        } else {
            unsafe { libc::send(socket, buf, len, flags) }
        }
    }

    fn write(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *const c_void, size_t) -> ssize_t>,
        fd: c_int,
        buf: *const c_void,
        count: size_t,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(fd, buf, count)
        } else {
            unsafe { libc::write(fd, buf, count) }
        }
    }

    fn pwrite(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *const c_void, size_t, off_t) -> ssize_t>,
        fd: c_int,
        buf: *const c_void,
        count: size_t,
        offset: off_t,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(fd, buf, count, offset)
        } else {
            unsafe { libc::pwrite(fd, buf, count, offset) }
        }
    }

    fn writev(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *const iovec, c_int) -> ssize_t>,
        fd: c_int,
        iov: *const iovec,
        iovcnt: c_int,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(fd, iov, iovcnt)
        } else {
            unsafe { libc::writev(fd, iov, iovcnt) }
        }
    }

    fn pwritev(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *const iovec, c_int, off_t) -> ssize_t>,
        fd: c_int,
        iov: *const iovec,
        iovcnt: c_int,
        offset: off_t,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(fd, iov, iovcnt, offset)
        } else {
            unsafe { libc::pwritev(fd, iov, iovcnt, offset) }
        }
    }

    fn sendmsg(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *const msghdr, c_int) -> ssize_t>,
        fd: c_int,
        msg: *const msghdr,
        flags: c_int,
    ) -> ssize_t {
        if let Some(f) = fn_ptr {
            (f)(fd, msg, flags)
        } else {
            unsafe { libc::sendmsg(fd, msg, flags) }
        }
    }
}

#[cfg(target_os = "linux")]
pub trait LinuxNetSyscall: UnixNetSyscall {
    /// poll

    fn epoll_ctl(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, c_int, c_int, *mut libc::epoll_event) -> c_int>,
        epfd: c_int,
        op: c_int,
        fd: c_int,
        event: *mut libc::epoll_event,
    ) -> c_int {
        if let Some(f) = fn_ptr {
            (f)(epfd, op, fd, event)
        } else {
            unsafe { libc::epoll_ctl(epfd, op, fd, event) }
        }
    }

    /// socket

    fn accept4(
        &self,
        fn_ptr: Option<&extern "C" fn(c_int, *mut sockaddr, *mut socklen_t, c_int) -> c_int>,
        fd: c_int,
        addr: *mut sockaddr,
        len: *mut socklen_t,
        flg: c_int,
    ) -> c_int {
        if let Some(f) = fn_ptr {
            (f)(fd, addr, len, flg)
        } else {
            unsafe { libc::accept4(fd, addr, len, flg) }
        }
    }
}
