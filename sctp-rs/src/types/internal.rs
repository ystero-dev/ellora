//! Types used by different SCTP Internal APIs
//!
//! Most of these types are 'C' like `struct`s that are passed as parameters as a part of
//! performing certain SCTP related functionality using `libc::getsockopt` or `libc::setsockopt`.
//! Structures below are used by the implementation details and are not part of the public API.

use crate::{SctpAssociationId, SctpEvent};

// Structure used by `sctp_peeloff` (Section 9.2)
#[repr(C)]
#[derive(Default, Debug)]
pub(crate) struct SctpPeeloffArg {
    pub(crate) assoc_id: SctpAssociationId,
    pub(crate) sd: libc::c_int,
}

impl SctpPeeloffArg {
    pub(crate) fn from_assoc_id(assoc_id: SctpAssociationId) -> Self {
        Self { assoc_id, sd: 0 }
    }
}

// Structure used by `sctp_getpaddrs` and `sctp_getladdrs` (Section 9.3 and Section 9.4)
//
// This structure will always be used for 'getting' the values from the kernel.
#[repr(C)]
#[derive(Debug)]
pub(crate) struct SctpGetAddrs {
    pub(crate) assoc_id: SctpAssociationId,
    pub(crate) addr_count: libc::c_int,
    // Following type is just used as a place holder. The way this structure is 'always' used it is
    // we allocate memory and use that memory as a pointer to the structure and use the following
    // field to get the address of the following field and then use it as a `libc::sockaddr` and
    // iterate through those (see `getaddrs_internal`) and since this is never used as a part of
    // public API, our users don't have to worry about it.
    pub(crate) addrs: u8,
}

// Structure used for Subscribing to SCTP Events
#[repr(C)]
#[derive(Debug)]
pub(crate) struct SctpSubscribeEvent {
    pub(crate) assoc_id: SctpAssociationId,
    pub(crate) event: SctpEvent,
    pub(crate) on: bool,
}

// SCTP Initiation Structure (See Section 5.3.1 of RFC 6458)
#[repr(C)]
#[derive(Debug)]
pub(crate) struct SctpInitMsg {
    pub(crate) ostreams: u16,
    pub(crate) istreams: u16,
    pub(crate) retries: u16,
    pub(crate) timeout: u16, // in miliseconds
}

// Structure used by connectx (using SCTP_SOCKOPT_CONNECTX3). This is required to get the
// `assoc_id` in the case of non blocking sockets.
#[repr(C)]
#[derive(Debug)]
pub(crate) struct SctpConnectxParam {
    pub(crate) assoc_id: SctpAssociationId,
    pub(crate) addrs_size: libc::c_int,
    pub(crate) addrs: *mut u8,
}
