//! Types used by different SCTP Internal APIs
//!
//! Most of these types are 'C' like `struct`s that are passed as parameters as a part of
//! performing certain SCTP related functionality using `libc::getsockopt` or `libc::setsockopt`.
//! Structures below are used by the implementation details and are not part of the public API.

use crate::{AssociationId, Event};

// Structure used by `sctp_peeloff` (Section 9.2)
#[repr(C)]
#[derive(Default, Debug)]
pub(crate) struct PeeloffArg {
    pub(crate) assoc_id: AssociationId,
    pub(crate) sd: libc::c_int,
}

impl PeeloffArg {
    pub(crate) fn from_assoc_id(assoc_id: AssociationId) -> Self {
        Self { assoc_id, sd: 0 }
    }
}

// Structure used by `sctp_getpaddrs` and `sctp_getladdrs` (Section 9.3 and Section 9.4)
//
// This structure will always be used for 'getting' the values from the kernel.
#[repr(C)]
#[derive(Debug)]
pub(crate) struct GetAddrs {
    pub(crate) assoc_id: AssociationId,
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
pub(crate) struct SubscribeEvent {
    pub(crate) assoc_id: AssociationId,
    pub(crate) event: Event,
    pub(crate) on: bool,
}

// SCTP Initiation Structure (See Section 5.3.1 of RFC 6458)
#[repr(C)]
#[derive(Debug)]
pub(crate) struct InitMsg {
    pub(crate) ostreams: u16,
    pub(crate) istreams: u16,
    pub(crate) retries: u16,
    pub(crate) timeout: u16, // in miliseconds
}

// Structure used by connectx (using SCTP_SOCKOPT_CONNECTX3). This is required to get the
// `assoc_id` in the case of non blocking sockets.
#[repr(C)]
#[derive(Debug)]
pub(crate) struct ConnectxParam {
    pub(crate) assoc_id: AssociationId,
    pub(crate) addrs_size: libc::c_int,
    pub(crate) addrs: *mut u8,
}

// PeerAddress: Structure representing SCTP Peer Address.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PeerAddrInternal {
    pub assoc_id: AssociationId,
    pub address: libc::sockaddr_storage,
    pub state: i32,
    pub cwnd: u32,
    pub srtt: u32,
    pub rto: u32,
    pub mtu: u32,
}

// ConnStatusInternal: Status of an SCTP Connection
#[repr(C)]
#[derive(Clone)]
pub struct ConnStatusInternal {
    pub assoc_id: AssociationId,
    pub state: i32,
    pub rwnd: u32,
    pub unacked_data: u16,
    pub pending_data: u16,
    pub instreams: u16,
    pub outstreams: u16,
    pub fragmentation_pt: u32,
    pub peer_primary: PeerAddrInternal,
}

use std::convert::{TryFrom, TryInto};

use os_socketaddr::OsSocketAddr;

use crate::types::{ConnState, ConnStatus, PeerAddress};

impl TryFrom<PeerAddrInternal> for PeerAddress {
    type Error = std::io::Error;

    fn try_from(val: PeerAddrInternal) -> Result<Self, Self::Error> {
        let sa_family = val.address.ss_family;
        // Safety: address is valid and hence getting reference to it is valid.
        let address = unsafe {
            if sa_family as i32 == libc::AF_INET {
                let address = val.address;
                let os_socketaddr = OsSocketAddr::copy_from_raw(
                    &address as *const _ as *const libc::sockaddr,
                    std::mem::size_of::<libc::sockaddr_in>().try_into().unwrap(),
                );
                os_socketaddr.into_addr().unwrap()
            } else if sa_family as i32 == libc::AF_INET6 {
                let address = val.address;
                let os_socketaddr = OsSocketAddr::copy_from_raw(
                    &address as *const _ as *const libc::sockaddr,
                    std::mem::size_of::<libc::sockaddr_in6>()
                        .try_into()
                        .unwrap(),
                );
                os_socketaddr.into_addr().unwrap()
            } else {
                return Err(std::io::Error::from_raw_os_error(22));
            }
        };
        Ok(Self {
            assoc_id: val.assoc_id,
            address,
            state: val.state,
            cwnd: val.cwnd,
            srtt: val.srtt,
            rto: val.rto,
            mtu: val.mtu,
        })
    }
}

impl TryFrom<ConnStatusInternal> for ConnStatus {
    type Error = std::convert::Infallible;

    fn try_from(val: ConnStatusInternal) -> Result<Self, Self::Error> {
        Ok(Self {
            assoc_id: val.assoc_id,
            state: ConnState::from_i32(val.state),
            rwnd: val.rwnd,
            unacked_data: val.unacked_data,
            pending_data: val.pending_data,
            instreams: val.instreams,
            outstreams: val.outstreams,
            fragmentation_pt: val.fragmentation_pt,
            peer_primary: val.peer_primary.try_into().unwrap(),
        })
    }
}
