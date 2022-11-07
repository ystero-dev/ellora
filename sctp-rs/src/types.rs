//! Types used by different SCTP Internal APIs
//!
//! Most of these types are 'C' like `struct`s that are passed as parameters as a part of
//! performing certain SCTP related functionality using `libc::getsockopt` or `libc::setsockopt`.

/// SCTP Association ID Type
pub type SctpAssociationId = i32;

/// Flags used by `sctp_bindx`.
#[derive(Debug, Clone)]
pub enum BindxFlags {
    /// Add the addresses passed (corresponding to `SCTP_BINDX_ADD_ADDR`)
    Add,

    /// Remove the addresses passed (corresponding to `SCTP_BINDX_REM_ADDR`)
    Remove,
}

/// SocketToAssociation: One-to-Many or One-to-One style Socket
#[derive(Debug, Clone)]
pub enum SocketToAssociation {
    /// One Association per Socket
    OneToOne,

    /// Many Associations per Socket
    OneToMany,
}

// Structures below are used by the implementation details and are not part of the public API.

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
