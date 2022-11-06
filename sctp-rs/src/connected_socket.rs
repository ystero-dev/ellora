//! A Connected SCTP Socket. This is similar to `TCPStream`.

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

#[allow(unused)]
use crate::internal::*;
use crate::{types::SctpAssociationId, BindxFlags};

/// A `ConnectedSctpSocket`
#[derive(Debug)]
pub struct SctpConnectedSocket {
    inner: RawFd,
}

impl super::__InternalSCTP for SctpConnectedSocket {}

impl SctpConnectedSocket {
    /// Create SctpConnectedSocket from a `RawFd`
    pub fn from_rawfd(rawfd: RawFd) -> Self {
        Self { inner: rawfd }
    }

    /// Close the socket
    pub fn close(&self) -> std::io::Result<()> {
        unimplemented!();
    }

    /// Shutdown on the socket
    pub fn shutdown(&self, _how: std::net::Shutdown) -> std::io::Result<()> {
        unimplemented!();
    }

    /// Section 9.1 RFC 6458
    ///
    /// Since this is an optional feature, this should simply return an error!
    pub fn sctp_bindx(&self, _addrs: &[SocketAddr], _flags: BindxFlags) -> std::io::Result<()> {
        Err(std::io::Error::from_raw_os_error(95))
    }

    /// Section 9.3 RFC 6458
    ///
    /// Get's the Peer Addresses for the association.
    pub fn sctp_getpaddrs(&self, assoc_id: SctpAssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getpaddrs_internal(self.inner, assoc_id)
    }

    /// Section 9.5 RFC 6458
    ///
    /// Get's the Local Addresses for the association.
    pub fn sctp_getladdrs(&self, assoc_id: SctpAssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getladdrs_internal(self.inner, assoc_id)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn bindx_not_supported() {
        let connected = crate::SctpConnectedSocket::from_rawfd(42);
        let bindaddr = "127.0.0.1:8080".parse().unwrap();
        let result = connected.sctp_bindx(&[bindaddr], crate::BindxFlags::Add);
        assert!(result.is_err(), "{:#?}", result.ok().unwrap());
    }
}
