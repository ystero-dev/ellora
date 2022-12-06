//! Listening SCTP Socket

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

use tokio::io::unix::AsyncFd;

#[allow(unused)]
use crate::internal::*;
use crate::{
    types::AssociationId, BindxFlags, ConnStatus, ConnectedSocket, Event, NotificationOrData,
    SendData, SubscribeEventAssocId,
};

/// A structure representing a socket that is listening for incoming SCTP Connections.
///
/// This structure is created by an [`crate::Socket`] when it is bound to local address(es)
/// and is waiting for incoming connections by calling the `listen` on the socket. The original
/// [`crate::Socket`] is consumed when this structure is created. See
/// [`crate::Socket::listen`] for more details.
pub struct Listener {
    inner: AsyncFd<RawFd>,
}

impl Listener {
    /// Accept on a given socket (valid only for `OneToOne` type sockets).
    pub async fn accept(&self) -> std::io::Result<(ConnectedSocket, SocketAddr)> {
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
    ///
    /// This call is successful only for UDP style one to many sockets. This is like
    /// `[crate::Listener::accept`] where peeled off socket behaves like a stand alone
    /// one-to-one socket.
    pub fn sctp_peeloff(&self, assoc_id: AssociationId) -> std::io::Result<ConnectedSocket> {
        sctp_peeloff_internal(&self.inner, assoc_id)
    }

    /// Get Peer Address(es) for the given Association ID. See: Section 9.3 RFC 6458
    ///
    /// This function is supported on the [`Listener`] because in the case of One to Many
    /// associations that are not peeled off, we are performing IO operations on the listening
    /// socket itself.
    pub fn sctp_getpaddrs(&self, assoc_id: AssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getpaddrs_internal(&self.inner, assoc_id)
    }

    /// Get's the Local Addresses for the association. See: Section 9.4 RFC 6458
    pub fn sctp_getladdrs(&self, assoc_id: AssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getladdrs_internal(&self.inner, assoc_id)
    }

    /// Receive Data or Notification from the listening socket.
    ///
    /// In the case of One-to-many sockets, it is possible to receive on the listening socket,
    /// without explicitly 'accept'ing or 'peeling off' the socket. The internal API used to
    /// receive the data is also the API used to receive notifications. This function returns
    /// either the notification (which the user should have subscribed for) or the data.
    pub async fn sctp_recv(&self) -> std::io::Result<NotificationOrData> {
        sctp_recvmsg_internal(&self.inner).await
    }

    /// Send Data and Anciliary data if any on the SCTP Socket.
    ///
    /// SCTP supports sending the actual SCTP message together with sending any anciliary data on
    /// the SCTP association. The anciliary data is optional.
    pub async fn sctp_send(&self, to: SocketAddr, data: SendData) -> std::io::Result<()> {
        sctp_sendmsg_internal(&self.inner, Some(to), data).await
    }

    /// Subscribe to a given SCTP Event on the given socket. See section 6.2.1 of RFC6458.
    ///
    /// SCTP allows receiving notifications about the changes to SCTP associations etc from the
    /// user space. For these notification events to be received, this API is used to subsribe for
    /// the events while receiving the data on the SCTP Socket.
    pub fn sctp_subscribe_event(
        &self,
        event: Event,
        assoc_id: SubscribeEventAssocId,
    ) -> std::io::Result<()> {
        sctp_subscribe_event_internal(&self.inner, event, assoc_id, true)
    }

    /// Unsubscribe from a given SCTP Event on the given socket. See section 6.2.1 of RFC6458.
    ///
    /// See [`sctp_subscribe_event`][`Self::sctp_subscribe_event`] for further details.
    pub fn sctp_unsubscribe_event(
        &self,
        event: Event,
        assoc_id: SubscribeEventAssocId,
    ) -> std::io::Result<()> {
        sctp_subscribe_event_internal(&self.inner, event, assoc_id, false)
    }

    /// Setup parameters for a new association.
    ///
    /// To specify custom parameters for a new association this API is used.
    pub fn sctp_setup_init_params(
        &self,
        ostreams: u16,
        istreams: u16,
        retries: u16,
        timeout: u16,
    ) -> std::io::Result<()> {
        sctp_setup_init_params_internal(&self.inner, ostreams, istreams, retries, timeout)
    }

    /// Request to receive `RcvInfo` ancillary data.
    ///
    /// SCTP allows receiving ancillary data about the curent data received on the given socket.
    /// This API is used to obtain receive side additional info when the data is to be received.
    pub fn sctp_request_rcvinfo(&self, on: bool) -> std::io::Result<()> {
        request_rcvinfo_internal(&self.inner, on)
    }

    /// Request to receive `NxtInfo` ancillary data.
    ///
    /// SCTP allows receiving ancillary data about the curent data received on the given socket.
    /// This API is used to obtain information about the next datagram that will be received.
    pub fn sctp_request_nxtinfo(&self, on: bool) -> std::io::Result<()> {
        request_nxtinfo_internal(&self.inner, on)
    }

    /// Get the status of the connection associated with the association ID.
    pub fn sctp_get_status(&self, assoc_id: AssociationId) -> std::io::Result<ConnStatus> {
        sctp_get_status_internal(&self.inner, assoc_id)
    }

    // functions not part of public APIs
    pub(crate) fn from_rawfd(fd: RawFd) -> std::io::Result<Self> {
        Ok(Self {
            inner: AsyncFd::new(fd)?,
        })
    }
}

impl Drop for Listener {
    // Drop for `Listener`. We close the `inner` RawFd
    fn drop(&mut self) {
        close_internal(&self.inner);
    }
}
