//! Listening SCTP Socket

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

use tokio::io::unix::AsyncFd;

#[allow(unused)]
use crate::internal::*;
use crate::{
    types::SctpAssociationId, BindxFlags, SctpConnectedSocket, SctpEvent, SctpNotificationOrData,
    SctpSendData, SctpStatus, SubscribeEventAssocId,
};

/// A structure representing a socket that is listening for incoming SCTP Connections.
///
/// This structure is created by an [`crate::SctpSocket`] when it is bound to local address(es)
/// and is waiting for incoming connections by calling the `listen` on the socket. The original
/// [`crate::SctpSocket`] is consumed when this structure is created. See
/// [`crate::SctpSocket::listen`] for more details.
pub struct SctpListener {
    inner: AsyncFd<RawFd>,
}

impl SctpListener {
    /// Accept on a given socket (valid only for `OneToOne` type sockets).
    pub async fn accept(&self) -> std::io::Result<(SctpConnectedSocket, SocketAddr)> {
        accept_internal(&self.inner).await
    }

    /// Shutdown on the socket
    pub fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        shutdown_internal(&self.inner, how)
    }

    /// Binds to one or more local addresses. See: Section 9.1 RFC 6458
    ///
    /// It is possible to call `sctp_bindx` on an already 'bound' (that is 'listen'ing socket.)
    pub fn sctp_bindx(&self, addrs: &[SocketAddr], flags: BindxFlags) -> std::io::Result<()> {
        sctp_bindx_internal(&self.inner, addrs, flags)
    }

    /// Peels off a connected SCTP association from the listening socket. See: Section 9.2 RFC 6458
    pub fn sctp_peeloff(
        &self,
        assoc_id: SctpAssociationId,
    ) -> std::io::Result<SctpConnectedSocket> {
        sctp_peeloff_internal(&self.inner, assoc_id)
    }

    /// Get Peer Address(es) for the given Association ID. See: Section 9.3 RFC 6458
    ///
    /// This function is supported on the [`SctpListener`] because in the case of One to Many
    /// associations that are not peeled off, we are performing IO operations on the listening
    /// socket itself.
    pub fn sctp_getpaddrs(&self, assoc_id: SctpAssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getpaddrs_internal(&self.inner, assoc_id)
    }

    /// Get's the Local Addresses for the association. See: Section 9.4 RFC 6458
    pub fn sctp_getladdrs(&self, assoc_id: SctpAssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getladdrs_internal(&self.inner, assoc_id)
    }

    /// Receive Data or Notification from the listening socket.
    ///
    /// This function returns either a notification or the data.
    pub async fn sctp_recv(&self) -> std::io::Result<SctpNotificationOrData> {
        sctp_recvmsg_internal(&self.inner).await
    }

    /// Send Data and Anciliary data if any on the SCTP Socket.
    ///
    /// This function returns the result of Sending data on the socket.
    pub async fn sctp_send(&self, to: SocketAddr, data: SctpSendData) -> std::io::Result<()> {
        sctp_sendmsg_internal(&self.inner, Some(to), data).await
    }

    /// Event Subscription for the socket.
    pub fn sctp_subscribe_event(
        &self,
        event: SctpEvent,
        assoc_id: SubscribeEventAssocId,
    ) -> std::io::Result<()> {
        sctp_subscribe_event_internal(&self.inner, event, assoc_id, true)
    }

    /// Event Unsubscription for the socket.
    pub fn sctp_unsubscribe_event(
        &self,
        event: SctpEvent,
        assoc_id: SubscribeEventAssocId,
    ) -> std::io::Result<()> {
        sctp_subscribe_event_internal(&self.inner, event, assoc_id, false)
    }

    /// Get's the status of the connection associated with the association ID
    pub fn sctp_get_status(&self, assoc_id: SctpAssociationId) -> std::io::Result<SctpStatus> {
        sctp_get_status_internal(&self.inner, assoc_id)
    }

    // functions not part of public APIs
    pub(crate) fn from_raw_fd(fd: RawFd) -> std::io::Result<Self> {
        Ok(Self {
            inner: AsyncFd::new(fd)?,
        })
    }
}

impl Drop for SctpListener {
    // Drop for `SctpListener`. We close the `inner` RawFd
    fn drop(&mut self) {
        close_internal(&self.inner);
    }
}
