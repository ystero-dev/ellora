//! A Connected SCTP Socket. This is similar to `TCPStream`.

use tokio::io::unix::AsyncFd;

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

#[allow(unused)]
use crate::internal::*;
use crate::{
    BindxFlags, SctpAssociationId, SctpEvent, SctpNotificationOrData, SctpSendData, SctpStatus,
    SubscribeEventAssocId,
};

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
    inner: AsyncFd<RawFd>,
}

impl SctpConnectedSocket {
    /// Creates new [`SctpConnectedSocket`] from a [`RawFd`][std::os::unix::io::RawFd].
    ///
    /// TODO: Remove this from Public API
    /// Although, this is available as public API as of now, likely the users are not required to
    /// use this. Mostly [`accept`][`crate::SctpListener::accept`] (in the case of One to One
    /// Socket to Association) or [`peeloff`][`crate::SctpListener::sctp_peeloff`] (in the case of
    /// One to Many Association) would use this API to create new [`SctpConnectedSocket`].
    pub fn from_rawfd(rawfd: RawFd) -> std::io::Result<Self> {
        // Make sure that the FD is set as non-blocking.
        set_fd_non_blocking(rawfd)?;

        Ok(Self {
            inner: AsyncFd::new(rawfd)?,
        })
    }

    /// Perform a TCP like half close. Note: however that the semantics for TCP and SCTP half close
    /// are different. See section 4.1.7 of RFC 6458 for details.
    pub fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        shutdown_internal(*self.inner.get_ref(), how)
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
        sctp_getpaddrs_internal(*self.inner.get_ref(), assoc_id)
    }

    /// Get Local addresses for the association. See section 9.5 RFC 6458.
    pub fn sctp_getladdrs(&self, assoc_id: SctpAssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getladdrs_internal(*self.inner.get_ref(), assoc_id)
    }

    /// Receive Data or Notification from the listening socket.
    ///
    /// This function returns either a notification or the data.
    pub fn sctp_recv(&self) -> std::io::Result<SctpNotificationOrData> {
        sctp_recvmsg_internal(*self.inner.get_ref())
    }

    /// Send Data and Anciliary data if any on the SCTP Socket.
    ///
    /// This function returns the result of Sending data on the socket.
    pub fn sctp_send(&self, data: SctpSendData) -> std::io::Result<()> {
        sctp_sendmsg_internal(*self.inner.get_ref(), None, data)
    }

    /// Event Subscription for the socket.
    pub fn sctp_subscribe_event(
        &self,
        event: SctpEvent,
        assoc_id: SubscribeEventAssocId,
    ) -> std::io::Result<()> {
        sctp_subscribe_event_internal(*self.inner.get_ref(), event, assoc_id, true)
    }

    /// Event Unsubscription for the socket.
    pub fn sctp_unsubscribe_event(
        &self,
        event: SctpEvent,
        assoc_id: SubscribeEventAssocId,
    ) -> std::io::Result<()> {
        sctp_subscribe_event_internal(*self.inner.get_ref(), event, assoc_id, false)
    }

    /// Request to receive `SctpRcvInfo` ancillary data
    pub fn sctp_request_rcvinfo(&self, on: bool) -> std::io::Result<()> {
        request_rcvinfo_internal(*self.inner.get_ref(), on)
    }

    /// Request to receive `SctpNxtInfo` ancillary data
    pub fn sctp_request_nxtinfo(&self, on: bool) -> std::io::Result<()> {
        request_nxtinfo_internal(*self.inner.get_ref(), on)
    }

    /// Get's the status of the connection associated with the association ID
    pub fn sctp_get_status(&self, assoc_id: SctpAssociationId) -> std::io::Result<SctpStatus> {
        sctp_get_status_internal(*self.inner.get_ref(), assoc_id)
    }
}

impl Drop for SctpConnectedSocket {
    // Drop for `SctpConnectedSocket`. We close the `inner` RawFd
    fn drop(&mut self) {
        close_internal(*self.inner.get_ref());
    }
}
