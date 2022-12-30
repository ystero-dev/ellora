//! Actual implementation of the API Calls
//!
//! Nothing in this module should be public API as this module contains `unsafe` code that uses
//! `libc` and internal `libc` structs and function calls.

use tokio::io::unix::AsyncFd;

use std::convert::TryInto;
use std::net::SocketAddr;
use std::os::unix::io::{AsRawFd, RawFd};

use os_socketaddr::OsSocketAddr;

use crate::types::internal::{ConnectxParam, GetAddrs, InitMsg, SubscribeEvent};
use crate::{
    AssociationChange, AssociationId, BindxFlags, CmsgType, ConnStatus, ConnectedSocket, Event,
    Listener, Notification, NotificationOrData, NxtInfo, RcvInfo, ReceivedData, SendData, SendInfo,
    SubscribeEventAssocId,
};

#[allow(unused)]
use super::consts::*;

static SOL_SCTP: libc::c_int = 132;

// Implementation of `sctp_bindx` using `libc::setsockopt`
pub(crate) fn sctp_bindx_internal(
    fd: &AsyncFd<RawFd>,
    addrs: &[SocketAddr],
    flags: BindxFlags,
) -> std::io::Result<()> {
    log::debug!("Binding following addresses to socket: {:#?}", addrs);

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

    log::trace!(
        "addrs_len: {}, addrs_u8: {:?}, flags: {}",
        addrs_len,
        addrs_u8,
        flags
    );

    // Safety: The passed vector is valid during the function call and hence the passed reference
    // to raw data is valid.
    unsafe {
        let result = libc::setsockopt(
            *fd.get_ref(),
            SOL_SCTP,
            flags,
            addrs_u8.as_ptr() as *const _ as *const libc::c_void,
            addrs_len as libc::socklen_t,
        );

        if result < 0 {
            log::error!(
                "Error: {} during `sctp_bindx` using `setsockopt`.",
                std::io::Error::last_os_error()
            );
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

// Implementation of `sctp_peeloff` using `libc::getsockopt`
pub(crate) fn sctp_peeloff_internal(
    fd: &AsyncFd<RawFd>,
    assoc_id: AssociationId,
) -> std::io::Result<ConnectedSocket> {
    log::debug!("Peeling off socket for Association ID: {:?}", assoc_id);

    use crate::types::internal::PeeloffArg;

    let mut peeloff_arg = PeeloffArg::from_assoc_id(assoc_id);
    let mut peeloff_size: libc::socklen_t = std::mem::size_of::<PeeloffArg>() as libc::socklen_t;

    // Safety: Pointer to `peeloff_arg` and `peeloff_size` is valid as the variable is still in the
    // scope
    unsafe {
        let peeloff_arg_ptr = std::ptr::addr_of_mut!(peeloff_arg);
        let peeloff_size_ptr = std::ptr::addr_of_mut!(peeloff_size);
        let result = libc::getsockopt(
            *fd.get_ref(),
            SOL_SCTP,
            SCTP_SOCKOPT_PEELOFF,
            peeloff_arg_ptr as *mut _ as *mut libc::c_void,
            peeloff_size_ptr as *mut _ as *mut libc::socklen_t,
        );
        if result < 0 {
            log::error!(
                "Error: {} during `sctp_peeloff` using `getsockopt`.",
                std::io::Error::last_os_error()
            );
            Err(std::io::Error::last_os_error())
        } else {
            let rawfd = peeloff_arg.sd.as_raw_fd();

            log::debug!("Setting peeled off socket to non-blocking.");
            set_fd_non_blocking(rawfd)?;

            ConnectedSocket::from_rawfd(rawfd)
        }
    }
}

// Implementation of `socket` using `libc::socket`.
//
// Based on the type of the requested socket, we pass different `type` parameter to actual
// `libc::socket` call. See section 3.1.1 and section 4.1.1 of RFC 6458.
pub(crate) fn sctp_socket_internal(
    domain: libc::c_int,
    assoc: crate::SocketToAssociation,
) -> std::io::Result<RawFd> {
    unsafe {
        let rawfd = match assoc {
            crate::SocketToAssociation::OneToOne => {
                log::debug!("Creating TCP Style Socket.");
                libc::socket(domain, libc::SOCK_STREAM, libc::IPPROTO_SCTP)
            }
            crate::SocketToAssociation::OneToMany => {
                log::debug!("Creating UDP Style Socket.");
                libc::socket(domain, libc::SOCK_SEQPACKET, libc::IPPROTO_SCTP)
            }
        };

        log::debug!("Setting 'socket' to Non-blocking socket.");
        set_fd_non_blocking(rawfd)?;

        Ok(rawfd)
    }
}

// Implementation of `listen` using `libc::listen`
pub(crate) fn sctp_listen_internal(fd: AsyncFd<RawFd>, backlog: i32) -> std::io::Result<Listener> {
    unsafe {
        let rawfd = *fd.get_ref();
        let result = libc::listen(rawfd, backlog);

        if result < 0 {
            log::error!(
                "Error: {} during `sctp_listen`.",
                std::io::Error::last_os_error()
            );
            Err(std::io::Error::last_os_error())
        } else {
            Listener::from_rawfd(fd.into_inner())
        }
    }
}

// Implmentation of `sctp_getpaddrs` using `libc::getsockopt`
pub(crate) fn sctp_getpaddrs_internal(
    fd: &AsyncFd<RawFd>,
    assoc_id: AssociationId,
) -> std::io::Result<Vec<SocketAddr>> {
    sctp_getaddrs_internal(*fd.get_ref(), SCTP_GET_PEER_ADDRS, assoc_id)
}

// Implmentation of `sctp_getladdrs` using `libc::getsockopt`
pub(crate) fn sctp_getladdrs_internal(
    fd: &AsyncFd<RawFd>,
    assoc_id: AssociationId,
) -> std::io::Result<Vec<SocketAddr>> {
    sctp_getaddrs_internal(*fd.get_ref(), SCTP_GET_LOCAL_ADDRS, assoc_id)
}

// Actual function performing `sctp_getpaddrs` or `sctp_getladdrs`
fn sctp_getaddrs_internal(
    fd: RawFd,
    flags: libc::c_int,
    assoc_id: AssociationId,
) -> std::io::Result<Vec<SocketAddr>> {
    let addr_type = if flags == SCTP_GET_LOCAL_ADDRS {
        "local"
    } else {
        "peer"
    };
    log::debug!(
        "Getting {} Addresses for the SCTP Association {:?}",
        addr_type,
        assoc_id
    );

    let capacity = 256_usize;
    let mut addrs_buff: Vec<u8> = vec![0; capacity];
    let mut getaddrs_size: libc::socklen_t = capacity as libc::socklen_t;

    // Safety: `addrs_buff` has a reserved capacity of 4K bytes which should normally be sufficient
    // for most of the calls to get local or peer addresses. Even if it is not sufficient, the call
    // to `getsockopt` would return an error, thus the memory won't be overwritten.
    unsafe {
        let mut getaddrs_ptr = addrs_buff.as_mut_ptr() as *mut GetAddrs;
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
            log::error!(
                "Error: {} while getting {} addresses using  `getsockopt`.",
                addr_type,
                std::io::Error::last_os_error()
            );
            Err(std::io::Error::last_os_error())
        } else {
            let mut peeraddrs = vec![];

            // The call succeeded, we need to do a lot of ugly pointer arithmetic, first we get the
            // number of addresses of the peer `addr_count` written to by the call to `getsockopt`.
            let addr_count = (*getaddrs_ptr).addr_count;
            log::trace!("Got {} addresses", addr_count);

            let mut sockaddr_ptr = std::ptr::addr_of!((*getaddrs_ptr).addrs);
            for _ in 0..addr_count {
                // Now for each of the 'addresses', we try to get the family and then interpret
                // each of the addresses accordingly and update the pointer.
                let sa_family = (*(sockaddr_ptr as *const _ as *const libc::sockaddr)).sa_family;
                if sa_family as i32 == libc::AF_INET {
                    let os_socketaddr = OsSocketAddr::copy_from_raw(
                        sockaddr_ptr as *const _ as *const libc::sockaddr,
                        std::mem::size_of::<libc::sockaddr_in>().try_into().unwrap(),
                    );
                    let socketaddr = os_socketaddr.into_addr().unwrap();
                    log::trace!("Got IPv4 Address: {:#?}", socketaddr);
                    peeraddrs.push(socketaddr);
                    sockaddr_ptr = sockaddr_ptr
                        .offset(std::mem::size_of::<libc::sockaddr_in>().try_into().unwrap());
                } else if sa_family as i32 == libc::AF_INET6 {
                    let os_socketaddr = OsSocketAddr::copy_from_raw(
                        sockaddr_ptr as *const _ as *const libc::sockaddr,
                        std::mem::size_of::<libc::sockaddr_in6>()
                            .try_into()
                            .unwrap(),
                    );
                    let socketaddr = os_socketaddr.into_addr().unwrap();
                    log::trace!("Got IPv6 Address: {:#?}", socketaddr);
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

// Implementation of `sctp_connectx` using `getsockopt` and new API using `SCTP_SOCKOPT_CONNECTX3`.
pub(crate) async fn sctp_connectx_internal(
    fd: AsyncFd<RawFd>,
    addrs: &[SocketAddr],
) -> std::io::Result<(ConnectedSocket, AssociationId)> {
    let mut addrs_u8: Vec<u8> = vec![];

    log::debug!("Connecting to {:?} using `getsockopt`", addrs);

    for addr in addrs {
        let ossockaddr: OsSocketAddr = (*addr).into();
        let slice = ossockaddr.as_ref();
        addrs_u8.extend(slice);
    }

    let addrs_len = addrs_u8.len();

    let raw_fd = *fd.get_ref();
    // Safety: The passed vector is valid during the function call and hence the passed reference
    // to raw data is valid.
    unsafe {
        let mut params = ConnectxParam {
            assoc_id: 0,
            addrs_size: addrs_len.try_into().unwrap(),
            addrs: addrs_u8.as_mut_ptr(),
        };

        let mut params_size = std::mem::size_of::<ConnectxParam>() as libc::socklen_t;

        let result = libc::getsockopt(
            raw_fd,
            SOL_SCTP,
            SCTP_SOCKOPT_CONNECTX3,
            &mut params as *mut _ as *mut libc::c_void,
            &mut params_size as *mut _ as *mut libc::socklen_t,
        );

        if result < 0 {
            let last_error = std::io::Error::last_os_error();
            if last_error.raw_os_error() != Some(libc::EINPROGRESS) {
                log::error!(
                    "Error: '{}' while connecting using `getsockopt`.",
                    std::io::Error::last_os_error()
                );
                return Err(last_error);
            }
        }

        log::debug!("Waiting to connect...");
        let _guard = fd.writable().await?;
        log::debug!("Connected...");

        let sctp_status = sctp_get_status_internal(&fd, params.assoc_id);
        if let Err(e) = sctp_status {
            let err = if e.raw_os_error() != Some(libc::EINVAL) {
                e
            } else {
                log::error!("Received `EINVAL`, while getting status, returning `ECONNREFUSED`.");
                std::io::Error::from_raw_os_error(libc::ECONNREFUSED)
            };
            return Err(err);
        }

        log::debug!(
            "Socket State for Assoc ID: {},  {:#?}",
            params.assoc_id,
            sctp_status.unwrap().state
        );

        // We can (and should) now 'consume' the passed `fd` or else 'registration' of next
        // `ConnectedSocket` (during `AsyncFd::new` would fail. Consuming the `AsyncFd` would
        // de-register.)
        // Also, since this `fd` is the 'original' created with `socket` call, no need to set it to
        // non-blocking again.
        let rawfd = fd.into_inner();

        Ok((ConnectedSocket::from_rawfd(rawfd)?, params.assoc_id))
    }
}

// Implementation of `accept` - we just call the `libc::accept` allowing it to fail if the socket
// type is not the right one (UDP Style `SOCK_SEQPACKET`).
pub(crate) async fn accept_internal(
    fd: &AsyncFd<RawFd>,
) -> std::io::Result<(ConnectedSocket, SocketAddr)> {
    // Safety: Both `addrs_buff` and `addrs_len` are in the scope and hence are valid pointers.
    unsafe {
        let raw_fd = *fd.get_ref();

        // This is ugly for the following reasons - On the `SEQPACKET` sockets, we do not get
        // `readable` ready at all for the `accept`.  (Why not sure? Even when tried after sending
        // some dummy data to make sure we can recv on it.) Thus we try `accept` first for
        // `SEQPACKET` sockets, this `accept` would fail with `EINVAL` and for `STREAM` sockets,
        // this 'may' fail with `EWOULDBLOCK`. If it does, we wait for `readable` event again, in
        // the next iteration of the `loop`, we won't get `EWOULDBLOCK` and will actually `accept`.
        loop {
            // this should be enough to `accept` a connection normally `sockaddr`s maximum size is
            // 28 for the `sa_family` we care about.
            let mut addrs_buff: Vec<u8> = vec![0; 32];
            let mut addrs_len = addrs_buff.len();

            let result = {
                let addrs_len_ptr = std::ptr::addr_of_mut!(addrs_len);
                let addrs_buff_ptr = addrs_buff.as_mut_ptr();

                libc::accept(
                    raw_fd,
                    addrs_buff_ptr as *mut _ as *mut libc::sockaddr,
                    addrs_len_ptr as *mut _ as *mut libc::socklen_t,
                )
            };

            if result < 0 {
                let last_error = std::io::Error::last_os_error();
                if last_error.raw_os_error() != Some(libc::EWOULDBLOCK) {
                    log::error!(
                        "Error: '{}' while `accept`ing on the socket.",
                        std::io::Error::last_os_error()
                    );
                    return Err(last_error);
                }

                // We got an `EWOULDBLOCK` let's wait.
                let _guard = fd.readable().await?;
            } else {
                let os_socketaddr = OsSocketAddr::copy_from_raw(
                    addrs_buff.as_ptr() as *const _ as *const libc::sockaddr,
                    addrs_len.try_into().unwrap(),
                );
                log::trace!(
                    "fd: {}, result: {},  addrs_len: {}, addrs_u8: {:?}",
                    raw_fd,
                    result,
                    addrs_len,
                    addrs_buff,
                );
                let socketaddr = os_socketaddr.into_addr().unwrap();

                log::debug!("Setting 'accepted' socket to non-blocking.");
                set_fd_non_blocking(result as RawFd)?;

                return Ok((ConnectedSocket::from_rawfd(result as RawFd)?, socketaddr));
            }
        }
    }
}

// Shutdown implementation for `Listener` and `ConnectedSocket`.
pub(crate) fn shutdown_internal(
    fd: &AsyncFd<RawFd>,
    how: std::net::Shutdown,
) -> std::io::Result<()> {
    use std::net::Shutdown;

    log::debug!("Calling 'shutdown' on socket with flags: {:?}", how);
    let flags = match how {
        Shutdown::Read => libc::SHUT_RD,
        Shutdown::Write => libc::SHUT_WR,
        Shutdown::Both => libc::SHUT_RDWR,
    };

    // Safety: No real undefined behavior as long as fd is a valid fd and if fd is not a valid fd
    // the underlying systemcall will error.
    unsafe {
        let result = libc::shutdown(*fd.get_ref(), flags);
        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

// Implementation for the receive side for SCTP.
// TODO: Handle Control Message Header
pub(crate) async fn sctp_recvmsg_internal(
    fd: &AsyncFd<RawFd>,
) -> std::io::Result<NotificationOrData> {
    log::debug!("Receiving Message on the socket.");

    let mut recv_buffer = vec![0_u8; 4096];
    let mut recv_iov = libc::iovec {
        iov_base: recv_buffer.as_mut_ptr() as *mut _ as *mut libc::c_void,
        iov_len: recv_buffer.len(),
    };

    // Safety: wrapper over `libc` call. the size of the structures are wellknown.
    let msg_control_size = unsafe {
        libc::CMSG_SPACE(
            std::mem::size_of::<RcvInfo>() as u32 + std::mem::size_of::<NxtInfo>() as u32,
        )
    };
    //
    // Safety: recvmsg_hdr is valid in the current scope.
    unsafe {
        let rawfd = *fd.get_ref();

        loop {
            let mut guard = fd.readable().await?;

            let mut msg_control = vec![0u8; msg_control_size.try_into().unwrap()];
            let mut from_buffer = vec![0u8; 256];
            let mut recvmsg_header = libc::msghdr {
                msg_name: from_buffer.as_mut_ptr() as *mut _ as *mut libc::c_void,
                msg_namelen: from_buffer.len() as u32,
                msg_iov: &mut recv_iov,
                msg_iovlen: 1,
                msg_control: msg_control.as_mut_ptr() as *mut _ as *mut libc::c_void,
                msg_controllen: msg_control_size as usize,
                msg_flags: 0,
            };

            let flags = 0 as libc::c_int;
            let result = libc::recvmsg(rawfd, &mut recvmsg_header as *mut libc::msghdr, flags);
            if result < 0 {
                let last_error = std::io::Error::last_os_error();
                if last_error.kind() == std::io::ErrorKind::WouldBlock {
                    guard.clear_ready();
                } else {
                    return Err(last_error);
                }
            } else {
                let received_flags: u32 = recvmsg_header.msg_flags.try_into().unwrap();
                recv_buffer.truncate(result as usize);

                if received_flags & MSG_NOTIFICATION != 0 {
                    log::debug!("Received Notification.");
                    return Ok(NotificationOrData::Notification(notification_from_message(
                        &recv_buffer,
                    )));
                } else {
                    let mut rcv_info = None;
                    let mut nxt_info = None;
                    let mut cmsghdr = libc::CMSG_FIRSTHDR(&mut recvmsg_header as *mut libc::msghdr);
                    loop {
                        if cmsghdr.is_null() {
                            break;
                        }
                        if (*cmsghdr).cmsg_level != libc::IPPROTO_SCTP {
                            log::warn!("cmsg_level is not SCTP");
                            continue;
                        }

                        if (*cmsghdr).cmsg_type == CmsgType::RcvInfo as i32 {
                            let mut recv_info_internal = RcvInfo::default();
                            let cmsg_data = libc::CMSG_DATA(cmsghdr);
                            std::ptr::copy(
                                cmsg_data,
                                &mut recv_info_internal as *mut _ as *mut u8,
                                std::mem::size_of::<RcvInfo>(),
                            );
                            log::debug!("Received: RcvInfo: {:#?}", recv_info_internal);
                            rcv_info = Some(recv_info_internal);
                        }

                        if (*cmsghdr).cmsg_type == CmsgType::NxtInfo as i32 {
                            let mut nxt_info_internal = NxtInfo::default();
                            let cmsg_data = libc::CMSG_DATA(cmsghdr);
                            std::ptr::copy(
                                cmsg_data,
                                &mut nxt_info_internal as *mut _ as *mut u8,
                                std::mem::size_of::<NxtInfo>(),
                            );
                            log::debug!("Received: NxtInfo: {:#?}", nxt_info_internal);
                            nxt_info = Some(nxt_info_internal);
                        }

                        cmsghdr = libc::CMSG_NXTHDR(
                            msg_control.as_mut_ptr() as *mut _ as *mut libc::msghdr,
                            cmsghdr,
                        );
                    }

                    log::debug!("Received Data.");
                    return Ok(NotificationOrData::Data(ReceivedData {
                        payload: recv_buffer,
                        rcv_info,
                        nxt_info,
                    }));
                }
            }
        }
    }
}

// Implementation of the Send side for SCTP.
pub(crate) async fn sctp_sendmsg_internal(
    fd: &AsyncFd<RawFd>,
    to: Option<SocketAddr>,
    data: SendData,
) -> std::io::Result<()> {
    let mut send_iov = libc::iovec {
        iov_base: data.payload.as_ptr() as *mut libc::c_void,
        iov_len: data.payload.len(),
    };

    let (to_buffer, to_buffer_len) = if let Some(addr) = to {
        let os_sockaddr: OsSocketAddr = addr.into();
        (
            os_sockaddr.as_ptr() as *mut libc::c_void,
            os_sockaddr.capacity(),
        )
    } else {
        (std::ptr::null::<OsSocketAddr>() as *mut libc::c_void, 0)
    };

    // TODO: Support copy and other send info as well.
    let (msg_control, msg_control_size) = if data.snd_info.is_some() {
        // Safety: wrapper over `libc` call. the size of the structures are wellknown.
        let msg_control_size = unsafe { libc::CMSG_SPACE(std::mem::size_of::<SendInfo>() as u32) };
        let msg_control = vec![0u8; msg_control_size.try_into().unwrap()];
        (
            msg_control.as_ptr() as *mut libc::c_void,
            msg_control_size as usize,
        )
    } else {
        (
            std::ptr::null::<libc::cmsghdr>() as *mut libc::c_void,
            0_usize,
        )
    };

    let mut sendmsg_header = libc::msghdr {
        msg_name: to_buffer,
        msg_namelen: to_buffer_len,
        msg_iov: &mut send_iov,
        msg_iovlen: 1,
        msg_control,
        msg_controllen: msg_control_size,
        msg_flags: 0,
    };
    // Safety: sendmsg_hdr is valid in the current scope.
    unsafe {
        let _guard = fd.writable().await?;
        let rawfd = *fd.get_ref();

        let flags = 0 as libc::c_int;
        let result = libc::sendmsg(rawfd, &mut sendmsg_header as *mut libc::msghdr, flags);
        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

pub(crate) fn sctp_set_default_sendinfo_internal(
    fd: &AsyncFd<RawFd>,
    sendinfo: SendInfo,
) -> std::io::Result<()> {
    unsafe {
        let result = libc::setsockopt(
            *fd.get_ref(),
            SOL_SCTP,
            SCTP_DEFAULT_SNDINFO,
            &sendinfo as *const _ as *const libc::c_void,
            std::mem::size_of::<SendInfo>().try_into().unwrap(),
        );
        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn notification_from_message(data: &[u8]) -> Notification {
    let notification_type = u16::from_ne_bytes(data[0..2].try_into().unwrap());
    log::trace!(
        "notification_type: {:x}, SCTP_ASSOC_CHANGE: {:x}",
        notification_type,
        SCTP_ASSOC_CHANGE
    );
    match notification_type {
        SCTP_ASSOC_CHANGE => {
            log::debug!("SCTP_ASSOC_CHANGE Notification Received.");
            let assoc_change = AssociationChange {
                type_: u16::from_ne_bytes(data[0..2].try_into().unwrap()),
                flags: u16::from_ne_bytes(data[2..4].try_into().unwrap()),
                length: u32::from_ne_bytes(data[4..8].try_into().unwrap()),
                state: u16::from_ne_bytes(data[8..10].try_into().unwrap()),
                error: u16::from_ne_bytes(data[10..12].try_into().unwrap()),
                ob_streams: u16::from_ne_bytes(data[12..14].try_into().unwrap()),
                ib_streams: u16::from_ne_bytes(data[14..16].try_into().unwrap()),
                assoc_id: i32::from_ne_bytes(data[16..20].try_into().unwrap()),
                info: data[20..].into(),
            };
            Notification::AssociationChange(assoc_change)
        }
        _ => {
            log::debug!("Unsupported notification received.");
            Notification::Unsupported
        }
    }
}

// Implementation of Event Subscription
pub(crate) fn sctp_subscribe_event_internal(
    fd: &AsyncFd<RawFd>,
    event: Event,
    assoc_id: SubscribeEventAssocId,
    on: bool,
) -> std::io::Result<()> {
    let subscriber = SubscribeEvent {
        event,
        assoc_id: assoc_id.into(),
        on,
    };

    unsafe {
        let result = libc::setsockopt(
            *fd.get_ref(),
            SOL_SCTP,
            SCTP_EVENT,
            &subscriber as *const _ as *const libc::c_void,
            std::mem::size_of::<SubscribeEvent>().try_into().unwrap(),
        );
        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

// Setup initiation parameters
pub(crate) fn sctp_setup_init_params_internal(
    fd: &AsyncFd<RawFd>,
    ostreams: u16,
    istreams: u16,
    retries: u16,
    timeout: u16,
) -> std::io::Result<()> {
    log::debug!("Setting up `init_params` using `setsockopt`");
    let init_params = InitMsg {
        ostreams,
        istreams,
        retries,
        timeout,
    };

    unsafe {
        let result = libc::setsockopt(
            *fd.get_ref(),
            SOL_SCTP,
            SCTP_INITMSG,
            &init_params as *const _ as *const libc::c_void,
            std::mem::size_of::<InitMsg>().try_into().unwrap(),
        );
        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

// Enable/Disable reception of `RcvInfo` actual call.
pub(crate) fn request_rcvinfo_internal(fd: &AsyncFd<RawFd>, on: bool) -> std::io::Result<()> {
    log::debug!("Requesting `rcv_info` along with received data on the socket.");

    let enable: libc::socklen_t = u32::from(on);
    let enable_size = std::mem::size_of::<libc::socklen_t>();

    unsafe {
        let result = libc::setsockopt(
            *fd.get_ref(),
            SOL_SCTP,
            SCTP_RECVRCVINFO,
            &enable as *const _ as *const libc::c_void,
            enable_size.try_into().unwrap(),
        );

        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

// Enable/Disable reception of `NxtInfo` actual call.
pub(crate) fn request_nxtinfo_internal(fd: &AsyncFd<RawFd>, on: bool) -> std::io::Result<()> {
    log::debug!("Requesting `nxt_info` along with received data on the socket.");

    let enable: libc::socklen_t = u32::from(on);
    let enable_size = std::mem::size_of::<libc::socklen_t>();

    unsafe {
        let result = libc::setsockopt(
            *fd.get_ref(),
            SOL_SCTP,
            SCTP_RECVNXTINFO,
            &enable as *const _ as *const libc::c_void,
            enable_size.try_into().unwrap(),
        );

        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

// Get the status for the given Assoc ID
pub(crate) fn sctp_get_status_internal(
    fd: &AsyncFd<RawFd>,
    assoc_id: AssociationId,
) -> std::io::Result<ConnStatus> {
    log::debug!("Calling `sctp_get_status_internal`.");

    let status_ptr = std::mem::MaybeUninit::<ConnStatus>::zeroed();
    let mut status_size = std::mem::size_of::<ConnStatus>();

    unsafe {
        let mut sctp_status = status_ptr.assume_init();
        sctp_status.assoc_id = assoc_id;

        let result = libc::getsockopt(
            *fd.get_ref(),
            SOL_SCTP,
            SCTP_STATUS,
            &mut sctp_status as *mut _ as *mut libc::c_void,
            &mut status_size as *mut _ as *mut libc::socklen_t,
        );

        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(sctp_status)
        }
    }
}

fn set_fd_non_blocking(fd: RawFd) -> std::io::Result<()> {
    // Set Non Blocking
    unsafe {
        let result = libc::fcntl(fd, libc::F_GETFL, 0);
        if result < 0 {
            return Err(std::io::Error::last_os_error());
        }
        let flags = result | libc::O_NONBLOCK;
        let result = libc::fcntl(fd, libc::F_SETFL, flags);
        if result < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

// Close the socket
#[inline(always)]
pub(crate) fn close_internal(fd: &AsyncFd<RawFd>) {
    unsafe {
        _ = libc::close(*fd.get_ref());
    }
}
