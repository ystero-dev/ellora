#![allow(dead_code)]
//! Constants used by lower level `libc` APIs
//!
//! These constants come from `linux/sctp.h`

// Constants used by `sctp_bindx`
pub(crate) static SCTP_SOCKOPT_BINDX_ADD: libc::c_int = 100;
pub(crate) static SCTP_SOCKOPT_BINDX_REM: libc::c_int = 101;

// peel off a one to many socket
pub(crate) static SCTP_SOCKOPT_PEELOFF: libc::c_int = 102;

// get peer/localaddrs
pub(crate) static SCTP_GET_PEER_ADDRS: libc::c_int = 108;
pub(crate) static SCTP_GET_LOCAL_ADDRS: libc::c_int = 109;

// To connect to an SCTP server.
pub(crate) static SCTP_SOCKOPT_CONNECTX: libc::c_int = 110;

// To subscribe to SCTP Events
pub(crate) static SCTP_EVENT: libc::c_int = 127;

//
pub(crate) static MSG_NOTIFICATION: u32 = 0x8000;

// Notification Types Constants
pub(crate) const SCTP_ASSOC_CHANGE: u16 = (1 << 15) | 0x0001;

// Association Change States
pub(crate) const SCTP_COMM_UP: u16 = 0;
pub(crate) const SCTP_COMM_LOST: u16 = 1;
pub(crate) const SCTP_RESTART: u16 = 2;
pub(crate) const SCTP_SHUTDOWN_COMP: u16 = 3;
pub(crate) const SCTP_CANT_STR_ASSOC: u16 = 4;

// Init Message used for `setsockopt`
pub(crate) const SCTP_INITMSG: libc::c_int = 2;

// Receving RCVINFO and NXTINFO
pub(crate) const SCTP_RECVRCVINFO: libc::c_int = 32;
pub(crate) const SCTP_RECVNXTINFO: libc::c_int = 33;

// Constants related to `enum sctp_cmsg_type`
#[repr(i32)]
pub(crate) enum SctpCmsgType {
    SctpInit = 0,
    SctpSndRcv,
    SctpSndInfo,
    SctpRcvInfo,
    SctpNxtInfo,
    SctpPrInfo,
    SctpAuthInfo,
    SctpDstAddrV4,
    SctpDstAddrV6,
}
