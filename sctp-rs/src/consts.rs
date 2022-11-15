//! Constants used by lower level `libc` APIs
//!
//! These constants come from `linux/sctp.h`

// Constants used by `sctp_bindx`
pub(crate) static SCTP_EVENTS: libc::c_int = 11;
pub(crate) static SCTP_SOCKOPT_BINDX_ADD: libc::c_int = 100;
pub(crate) static SCTP_SOCKOPT_BINDX_REM: libc::c_int = 101;
pub(crate) static SCTP_SOCKOPT_PEELOFF: libc::c_int = 102;
pub(crate) static SCTP_GET_PEER_ADDRS: libc::c_int = 108;
pub(crate) static SCTP_GET_LOCAL_ADDRS: libc::c_int = 109;
pub(crate) static SCTP_SOCKOPT_CONNECTX: libc::c_int = 110;
