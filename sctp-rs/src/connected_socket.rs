//! A Connected SCTP Socket. This is similar to `TCPStream`.

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

#[allow(unused)]
use crate::internal::*;
use crate::{types::SctpAssociationId, BindxFlags};

/// A structure representing a Connected SCTP socket.
///
/// A Connected SCTP Socket is associated with one or more Peer associations (each of which is
/// identified by an Association ID). A Connected SCTP Socket will be created by an
/// [`SctpListener`][crate::SctpListener] when it calls an `accept` (in the case of One to One style sockets) or upon
/// receiving a `SCTP_COMM_UP` event in `SCTP_ASSOC_CHANGE` notification.
///
/// It is also possible to [`peeloff`][crate::SctpListener::sctp_peeloff] a socket from One to Many
/// listening socket and the peeled socket is an [`SctpConnectedSocket`].
#[derive(Debug)]
pub struct SctpConnectedSocket {
    inner: RawFd,
}

impl SctpConnectedSocket {
    /// Creates new [`SctpConnectedSocket`] from a [`RawFd`][std::os::unix::io::RawFd].
    ///
    /// Although, this is available as public API as of now, likely the users are not required to
    /// use this. Mostly [`accept`][`crate::SctpListener::accept`] (in the case of One to One
    /// Socket to Association) or [`peeloff`][`crate::SctpListener::sctp_peeloff`] (in the case of
    /// One to Many Association) would use this API to create new [`SctpConnectedSocket`].
    pub fn from_rawfd(rawfd: RawFd) -> Self {
        Self { inner: rawfd }
    }

    /// Perform a TCP like half close. Note: however that the semantics for TCP and SCTP half close
    /// are different. See section 4.1.7 of RFC 6458 for details.
    pub fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        shutdown_internal(self.inner, how)
    }

    /// Bind to addresses on the given socket. See Section 9.1 RFC 6458.
    ///
    /// For the connected sockets, this feature is optional and hence will *always* return
    /// `ENOTSUP(EOPNOTSUP)` error.
    pub fn sctp_bindx(&self, _addrs: &[SocketAddr], _flags: BindxFlags) -> std::io::Result<()> {
        Err(std::io::Error::from_raw_os_error(95))
    }

    /// Get Peer addresses for the association. See Section 9.3 RFC 6458.
    pub fn sctp_getpaddrs(&self, assoc_id: SctpAssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getpaddrs_internal(self.inner, assoc_id)
    }

    /// Get Local addresses for the association. See section 9.5 RFC 6458.
    pub fn sctp_getladdrs(&self, assoc_id: SctpAssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getladdrs_internal(self.inner, assoc_id)
    }
}

impl Drop for SctpConnectedSocket {
    // Drop for `SctpConnectedSocket`. We close the `inner` RawFd
    fn drop(&mut self) {
        unsafe { _ = libc::close(self.inner) }
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
