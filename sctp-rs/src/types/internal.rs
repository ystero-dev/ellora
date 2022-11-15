// Structures below are used by the implementation details and are not part of the public API.

use crate::SctpAssociationId;

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

// Structure used for subscribing to Event notifications.
//
// See Also: `struct sctp_event_subscribe` inside `/usr/include/linux/sctp.h`
//
// TODO: Add a `builder` structure for this.
#[repr(C)]
#[derive(Debug, Default)]
pub(crate) struct SctpEventSubscribe {
    pub(crate) data_io: u8,
    pub(crate) association: u8,
    pub(crate) address: u8,
    pub(crate) send_failure: u8,
    pub(crate) peer_error: u8,
    pub(crate) shutdown: u8,
    pub(crate) partial_delivery: u8,
    pub(crate) adaptation_layer: u8,
    pub(crate) authentication: u8,
    pub(crate) sender_dry: u8,
    pub(crate) stream_reset: u8,
    pub(crate) association_reset: u8,
    pub(crate) stream_change: u8,
    pub(crate) send_failure_event: u8,
}
