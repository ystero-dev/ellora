//! Types used by different SCTP Internal APIs
//!
//! Most of these types are 'C' like `struct`s that are passed as parameters as a part of
//! performing certain SCTP related functionality using `libc::getsockopt` or `libc::setsockopt`.

pub type SctpAssociationId = i32;

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
    pub(crate) addr_count: u32,
    pub(crate) addrs: *mut u8,
}
