//! Actual implementation of the API Calls
//!
//! Nothing in this module should be public API as this module contains `unsafe` code that uses
//! `libc` and internal `libc` structs and function calls.

use std::convert::TryInto;
use std::net::SocketAddr;
use std::os::unix::io::{AsRawFd, RawFd};

use os_socketaddr::OsSocketAddr;

use crate::{types::SctpGetAddrs, BindxFlags, SctpAssociationId, SctpConnectedSocket};

#[allow(unused)]
use super::consts::*;

static SOL_SCTP: libc::c_int = 132;

/// Implementation of `sctp_bindx` using `libc::setsockopt`
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

/// Implementation of `sctp_peeloff` using `libc::getsockopt`
pub(crate) fn sctp_peeloff_internal(
    fd: RawFd,
    assoc_id: SctpAssociationId,
) -> std::io::Result<RawFd> {
    use crate::types::SctpPeeloffArg;

    let mut peeloff_arg = SctpPeeloffArg::from_assoc_id(assoc_id);
    let mut peeloff_size: libc::socklen_t =
        std::mem::size_of::<SctpPeeloffArg>() as libc::socklen_t;

    // Safety Pointer to `peeloff_arg` and `peeloff_size` is valid as the variable is still in the
    // scope
    unsafe {
        let peeloff_arg_ptr = std::ptr::addr_of_mut!(peeloff_arg);
        let peeloff_size_ptr = std::ptr::addr_of_mut!(peeloff_size);
        let result = libc::getsockopt(
            fd,
            SOL_SCTP,
            SCTP_SOCKOPT_PEELOFF,
            peeloff_arg_ptr as *mut _ as *mut libc::c_void,
            peeloff_size_ptr as *mut _ as *mut libc::socklen_t,
        );
        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(peeloff_arg.sd.as_raw_fd())
        }
    }
}

/// Implementation of `socket` using `libc::socket`.
///
/// Based on the type of the requested socket, we pass different `type` parameter to actual
/// `libc::socket` call. See section 3.1.1 and section 4.1.1 of RFC 6458.
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

/// Implementation of `listen` using `libc::listen`
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

/// Implmentation of `sctp_getpaddrs` using `libc::getsockopt`
pub(crate) fn sctp_getpaddrs_internal(
    fd: RawFd,
    assoc_id: SctpAssociationId,
) -> std::io::Result<Vec<SocketAddr>> {
    sctp_getaddrs_internal(fd, SCTP_GET_PEER_ADDRS, assoc_id)
}

/// Implmentation of `sctp_getladdrs` using `libc::getsockopt`
pub(crate) fn sctp_getladdrs_internal(
    fd: RawFd,
    assoc_id: SctpAssociationId,
) -> std::io::Result<Vec<SocketAddr>> {
    sctp_getaddrs_internal(fd, SCTP_GET_LOCAL_ADDRS, assoc_id)
}

// Actual function performing `sctp_getpaddrs` or `sctp_getladdrs`
fn sctp_getaddrs_internal(
    fd: RawFd,
    flags: libc::c_int,
    assoc_id: SctpAssociationId,
) -> std::io::Result<Vec<SocketAddr>> {
    let capacity = 4096 as usize;
    let mut addrs_buff = Vec::<u8>::with_capacity(capacity);
    let mut getaddrs_size: libc::socklen_t = capacity as libc::socklen_t;

    // Safety: `addrs_buff` has a reserved capacity of 4K bytes which should normally be sufficient
    // for most of the calls to get local or peer addresses. Even if it is not sufficient, the call
    // to `getsockopt` would return an error, thus the memory won't be overwritten.
    unsafe {
        let mut getaddrs_ptr = addrs_buff.as_mut_ptr() as *mut SctpGetAddrs;
        (*getaddrs_ptr).assoc_id = assoc_id;
        let getaddrs_size_ptr = std::ptr::addr_of_mut!(getaddrs_size);
        let result = libc::getsockopt(
            fd,
            SOL_SCTP,
            flags,
            getaddrs_ptr as *mut _ as *mut libc::c_void,
            getaddrs_size_ptr as *mut _ as *mut libc::socklen_t,
        );
        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            let mut peeraddrs = vec![];

            // The call succeeded, we need to do a lot of ugly pointer arithmetic, first we get the
            // number of addresses of the peer `addr_count` written to by the call to `getsockopt`.
            let addr_count = (*getaddrs_ptr).addr_count;
            let mut sockaddr_ptr = (*getaddrs_ptr).addrs;
            for _ in 0..addr_count {
                // Now for each of the 'addresses', we try to get the family and then interpret
                // each of the addresses accordingly and update the pointer.
                let sa_family = (*(sockaddr_ptr as *const libc::sockaddr)).sa_family;
                if sa_family as i32 == libc::AF_INET {
                    let os_socketaddr = OsSocketAddr::from_raw_parts(
                        sockaddr_ptr,
                        std::mem::size_of::<libc::sockaddr_in>().try_into().unwrap(),
                    );
                    let socketaddr = os_socketaddr.into_addr().unwrap();
                    peeraddrs.push(socketaddr);
                    sockaddr_ptr = sockaddr_ptr
                        .offset(std::mem::size_of::<libc::sockaddr_in>().try_into().unwrap());
                } else if sa_family as i32 == libc::AF_INET6 {
                    let os_socketaddr = OsSocketAddr::from_raw_parts(
                        sockaddr_ptr,
                        std::mem::size_of::<libc::sockaddr_in6>()
                            .try_into()
                            .unwrap(),
                    );
                    let socketaddr = os_socketaddr.into_addr().unwrap();
                    peeraddrs.push(socketaddr);
                    sockaddr_ptr = sockaddr_ptr.offset(
                        std::mem::size_of::<libc::sockaddr_in6>()
                            .try_into()
                            .unwrap(),
                    );
                } else {
                    // Unsupported Family - should never come here.
                    return Err(std::io::Error::from_raw_os_error(22));
                }
            }
            Ok(peeraddrs)
        }
    }
}

/// Implementation of `sctp_connectx` using setsockopt.
pub(crate) fn sctp_connectx_internal(
    fd: RawFd,
    addrs: &[SocketAddr],
) -> std::io::Result<(SctpConnectedSocket, SctpAssociationId)> {
    let mut addrs_u8: Vec<u8> = vec![];

    for addr in addrs {
        let ossockaddr: OsSocketAddr = (*addr).into();
        let slice = ossockaddr.as_ref();
        addrs_u8.extend(slice);
    }

    let addrs_len = addrs_u8.len();

    // Safety: The passed vector is valid during the function call and hence the passed reference
    // to raw data is valid.
    unsafe {
        let result = libc::setsockopt(
            fd,
            SOL_SCTP,
            SCTP_SOCKOPT_CONNECTX,
            addrs_u8.as_ptr() as *const _ as *const libc::c_void,
            addrs_len as libc::socklen_t,
        );

        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok((
                SctpConnectedSocket::from_rawfd(fd),
                result as SctpAssociationId,
            ))
        }
    }
}
