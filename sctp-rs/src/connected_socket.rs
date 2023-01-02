//! A Connected SCTP Socket. This is similar to `TCPStream`.

use tokio::io::unix::AsyncFd;

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

#[allow(unused)]
use crate::internal::*;
use crate::{
    AssociationId, BindxFlags, ConnStatus, Event, NotificationOrData, SendData, SendInfo,
    SubscribeEventAssocId,
};

/// A structure representing a Connected SCTP socket.
///
/// A Connected SCTP Socket is associated with one or more Peer associations (each of which is
/// identified by an Association ID). A Connected SCTP Socket will be created by an
/// [`Listener`][crate::Listener] when it calls an `accept` (in the case of One to One style
/// sockets) or upon receiving a `SCTP_COMM_UP` event in `SCTP_ASSOC_CHANGE` notification.
///
/// It is also possible to [`peeloff`][crate::Listener::sctp_peeloff] a socket from One to Many
/// listening socket and the peeled socket is an [`ConnectedSocket`].
#[derive(Debug)]
pub struct ConnectedSocket {
    inner: AsyncFd<RawFd>,
}

impl ConnectedSocket {
    /// Creates new [`ConnectedSocket`] from a [`RawFd`][std::os::unix::io::RawFd].
    ///
    /// TODO: Remove this from Public API
    /// Although, this is available as public API as of now, likely the users are not required to
    /// use this. Mostly [`accept`][`crate::Listener::accept`] (in the case of One to One
    /// Socket to Association) or [`peeloff`][`crate::Listener::sctp_peeloff`] (in the case of
    /// One to Many Association) would use this API to create new [`ConnectedSocket`].
    pub fn from_rawfd(rawfd: RawFd) -> std::io::Result<Self> {
        Ok(Self {
            inner: AsyncFd::new(rawfd)?,
        })
    }

    /// Perform a TCP like half close.
    ///
    /// Note: however that the semantics for TCP and SCTP half close are different. See section
    /// 4.1.7 of RFC 6458 for details.
    pub fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        shutdown_internal(&self.inner, how)
    }

    /// Bind to addresses on the given socket. See Section 9.1 RFC 6458.
    ///
    /// For the connected sockets, this feature is optional and hence will *always* return
    /// `ENOTSUP(EOPNOTSUP)` error.
    pub fn sctp_bindx(&self, _addrs: &[SocketAddr], _flags: BindxFlags) -> std::io::Result<()> {
        Err(std::io::Error::from_raw_os_error(95))
    }

    /// Get Peer addresses for the association. See Section 9.3 RFC 6458.
    pub fn sctp_getpaddrs(&self, assoc_id: AssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getpaddrs_internal(&self.inner, assoc_id)
    }

    /// Get Local addresses for the association. See section 9.5 RFC 6458.
    pub fn sctp_getladdrs(&self, assoc_id: AssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getladdrs_internal(&self.inner, assoc_id)
    }

    /// Receive Data or Notification from the connected socket.
    ///
    /// The internal API used to receive the data is also the API used to receive notifications.
    /// This function returns either the notification (which the user should have subscribed for)
    /// or the data.
    pub async fn sctp_recv(&self) -> std::io::Result<NotificationOrData> {
        sctp_recvmsg_internal(&self.inner).await
    }

    /// Send Data and Anciliary data if any on the SCTP Socket.
    ///
    /// SCTP supports sending the actual SCTP message together with sending any anciliary data on
    /// the SCTP association. The anciliary data is optional.
    pub async fn sctp_send(&self, data: SendData) -> std::io::Result<()> {
        sctp_sendmsg_internal(&self.inner, None, data).await
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

    /// Set Default `SendInfo` values for this socket.
    ///
    /// In the [`sctp_send`] API, an optional `SendInfo` is present, which can be used to specify the
    /// ancillary data along with the payload. Instead, a sender can chose to use this API to set
    /// the default `SendInfo` to be used while sending the data for this 'connected' socket.
    /// Note: This API is provided only for the [`ConnectedSocket`].
    pub fn sctp_set_default_sendinfo(&self, sendinfo: SendInfo) -> std::io::Result<()> {
        sctp_set_default_sendinfo_internal(&self.inner, sendinfo)
    }
}

impl Drop for ConnectedSocket {
    // Drop for `ConnectedSocket`. We close the `inner` RawFd
    fn drop(&mut self) {
        close_internal(&self.inner);
    }
}
