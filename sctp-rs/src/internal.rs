//! Actual implementation of the API Calls
//!
//! Nothing in this module should be public API as this module contains `unsafe` code that uses
//! `libc` and internal `libc` structs and function calls.

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

use os_socketaddr::OsSocketAddr;

use crate::BindxFlags;

#[allow(unused)]
use super::consts::*;

static SOL_SCTP: libc::c_int = 132;

pub(crate) fn sctp_bindx_internal(
    fd: RawFd,
    addrs: &[SocketAddr],
    flags: BindxFlags,
) -> std::io::Result<()> {
    let mut addrs_u8: Vec<u8> = vec![];

    for addr in addrs {
        let ossockaddr: OsSocketAddr = (*addr).into();
        let slice = ossockaddr.as_ref();
        addrs_u8.extend(slice);
    }

    let addrs_len = addrs_u8.len();

    let flags = match flags {
        BindxFlags::Add => SCTP_SOCKOPT_BINDX_ADD,
        BindxFlags::Remove => SCTP_SOCKOPT_BINDX_REM,
    };

    eprintln!(
        "addrs_len: {}, addrs_u8: {:?}, flags: {}",
        addrs_len, addrs_u8, flags
    );

    // Safety: The passed vector is valid during the function call and hence the passed reference
    // to raw data is valid.
    unsafe {
        let result = libc::setsockopt(
            fd,
            SOL_SCTP,
            flags,
            addrs_u8.as_ptr() as *const _ as *const libc::c_void,
            addrs_len as libc::socklen_t,
        );

        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

pub(crate) fn sctp_socket_internal(
    domain: libc::c_int,
    assoc: crate::SocketToAssociation,
) -> RawFd {
    unsafe {
        match assoc {
            crate::SocketToAssociation::OneToOne => {
                libc::socket(domain, libc::SOCK_STREAM, libc::IPPROTO_SCTP)
            }
            crate::SocketToAssociation::OneToMany => {
                libc::socket(domain, libc::SOCK_SEQPACKET, libc::IPPROTO_SCTP)
            }
        }
    }
}

pub(crate) fn sctp_listen_internal(fd: RawFd, backlog: i32) -> std::io::Result<()> {
    unsafe {
        let result = libc::listen(fd, backlog);

        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}
