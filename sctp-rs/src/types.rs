//! Types used by different SCTP Internal APIs
//!
//! Most of these types are 'C' like `struct`s that are passed as parameters as a part of
//! performing certain SCTP related functionality.

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
