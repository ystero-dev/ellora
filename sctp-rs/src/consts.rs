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
pub(crate) static SCTP_SOCKOPT_CONNECTX3: libc::c_int = 111;

// To subscribe to SCTP Events
pub(crate) static SCTP_EVENT: libc::c_int = 127;

//
pub(crate) static MSG_NOTIFICATION: u32 = 0x8000;

// Notification Types Constants
pub(crate) const SCTP_ASSOC_CHANGE: u16 = (1 << 15) | 0x0001;

// Init Message used for `setsockopt`
pub(crate) const SCTP_INITMSG: libc::c_int = 2;

// Receving RCVINFO and NXTINFO
pub(crate) const SCTP_RECVRCVINFO: libc::c_int = 32;
pub(crate) const SCTP_RECVNXTINFO: libc::c_int = 33;
pub(crate) const SCTP_DEFAULT_SNDINFO: libc::c_int = 34;

// Get SCTP Status
pub(crate) const SCTP_STATUS: libc::c_int = 14;
