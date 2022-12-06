//! Types used by the Public APIs

/// SCTP Association ID Type
pub type AssociationId = i32;

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
    /// One Association per Socket (TCP Style Socket.)
    OneToOne,

    /// Many Associations per Socket (UDP Style Socket.)
    OneToMany,
}

/// NotificationOrData: A type returned by a `sctp_recv` call.
#[derive(Debug, Clone)]
pub enum NotificationOrData {
    /// SCTP Notification received by an `sctp_recv` call.
    Notification(Notification),

    /// SCTP Data Received by an `sctp_recv` call.
    Data(ReceivedData),
}

/// Structure Representing SCTP Received Data.
///
/// This structure is returned by the `sctp_recv` API call. This contains in addition to 'received'
/// data, any ancillary data that is received during the underlying system call.
#[derive(Debug, Clone)]
pub struct ReceivedData {
    /// Received Message Payload.
    pub payload: Vec<u8>,

    /// Optional ancillary information about the received payload.
    pub rcv_info: Option<RcvInfo>,

    /// Optional ancillary information about the next call to `sctp_recv`.
    pub nxt_info: Option<NxtInfo>,
}

/// Structure Represnting Data to be Sent.
///
/// This structure contains actual paylod and optional ancillary data.
#[derive(Debug, Clone)]
pub struct SendData {
    /// Received Message Payload.
    pub payload: Vec<u8>,

    /// Optional ancillary information used to send the data.
    pub snd_info: Option<SendInfo>,
}

/// Structure representing Ancilliary Send Information (See Section 5.3.4 of RFC 6458)
#[derive(Debug, Default, Clone)]
pub struct SendInfo {
    /// Stream ID of the stream to send the data on.
    pub sid: u16,

    /// Flags to be used while sending the data.
    pub flags: u16,

    /// Application Protocol ID to be used while sending the data.
    pub ppid: u32,

    /// Opaque context to be used while sending the data.
    pub context: u32,

    /// Association ID of the SCTP Association to be used while sending the data.
    pub assoc_id: AssociationId,
}

/// Structure Representing Ancillary Receive Information (See Section 5.3.5 of RFC 6458)
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RcvInfo {
    /// Stream ID on which the data is received.
    pub sid: u16,

    /// Stream Sequence Number received in the data.
    pub ssn: u16,

    /// Flags for the received data.
    pub flags: u16,

    /// Application Protocol ID used by the sender while sending the data.
    pub ppid: u32,

    /// Transaction sequence number.
    pub tsn: u32,

    /// Cumulative sequence number.
    pub cumtsn: u32,

    /// Opaque context.
    pub context: u32,

    /// SCTP Association ID.
    pub assoc_id: AssociationId,
}

/// Structure representing Ancillary next information (See Section 5.3.5)
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct NxtInfo {
    /// Stream ID for the next received data.
    pub sid: u16,

    /// Flags for the next received data.
    pub flags: u16,

    /// Application protocol ID.
    pub ppid: u32,

    /// Length of the message to be used in the next `sctp_recv` call.
    pub length: u32,

    /// SCTP Association ID.
    pub assoc_id: AssociationId,
}

#[derive(Debug, Clone)]
/// An `enum` representing the notifications received on the SCTP Sockets.
pub enum Notification {
    /// Association Change Notification. See Section 6.1.1 of RFC 6458
    AssociationChange(AssociationChange),

    /// A Catchall Notification type for the Notifications that are not supported
    Unsupported,
}

/// AssociationChange: Structure returned as notification for Association Change.
///
/// To subscribe to this notification type, An application should call `sctp_subscribe_event` using
/// the [`Event`] type as [`Event::Association`].
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AssociationChange {
    /// Type of the Notification always `SCTP_ASSOC_CHAGE`
    pub type_: u16,

    /// Notification Flags. Unused currently.
    pub flags: u16,

    /// Length of the notification data.
    pub length: u32,

    /// Association Change state. See also [`AssocChangeState`].
    pub state: u16,

    /// Error when state is an error state and error information is available.
    pub error: u16,

    /// Maximum number of outbound streams.
    pub ob_streams: u16,

    /// Maximum number of inbound streams.
    pub ib_streams: u16,

    /// Association ID for the event.
    pub assoc_id: AssociationId,

    /// Additional data for the event.
    pub info: Vec<u8>,
}

/// Event: Used for Subscribing for SCTP Events
///
/// See [`sctp_subscribe_events`][`crate::Listener::sctp_subscribe_event`] for the usage.
#[repr(u16)]
#[derive(Debug, Clone)]
pub enum Event {
    /// Event to receive ancillary information with every `sct_recv`.
    DataIo = (1 << 15),

    /// Event related to association change.
    Association,

    /// Event related to peer address change.
    Address,

    /// Event related to send failure.
    SendFailure,

    /// Event related to error received from the peer.
    PeerError,

    /// Event related to indicate peer shutdown.
    Shutdown,

    /// Event related to indicate partial delivery.
    PartialDelivery,

    /// Event related to indicate peer's partial indication.
    AdaptationLayer,

    /// Authentication event.
    Authentication,

    /// Event related to sender having no outstanding user data.
    SenderDry,

    /// Event related to stream reset.
    StreamReset,

    /// Event related to association reset.
    AssociationReset,

    /// Event related to stream change.
    StreamChange,

    /// Send Failure Event indication. (The actual received information is different from the one
    /// received in the `SendFailed` event.)
    SendFailureEvent,
}

/// SubscribeEventAssocId: AssociationID Used for Event Subscription
///
/// Note: repr should be same as `AssociationId` (ie. `i32`)
#[repr(i32)]
pub enum SubscribeEventAssocId {
    /// Subscribe to Future Association IDs
    Future,

    /// Subscribe to Current Association IDs
    Current,

    /// Subscribe to ALL Association IDs
    All,

    /// Subscribe to Association ID with a given value.
    Value(AssociationId),
}

impl From<SubscribeEventAssocId> for AssociationId {
    fn from(value: SubscribeEventAssocId) -> Self {
        match value {
            SubscribeEventAssocId::Future => 0 as Self,
            SubscribeEventAssocId::Current => 1 as Self,
            SubscribeEventAssocId::All => 2 as Self,
            SubscribeEventAssocId::Value(v) => v,
        }
    }
}

/// Association Change States
#[repr(u16)]
#[derive(Clone, Debug)]
pub enum AssocChangeState {
    /// SCTP communication up.
    CommUp = 0,

    /// SCTP communication lost.
    CommLost,

    /// SCTP communication restarted.
    Restart,

    /// Shutdown complete.
    ShutdownComplete,

    /// Cannot start association.
    CannotStartAssoc,
}

/// Constants related to `enum sctp_cmsg_type`
#[repr(i32)]
pub enum CmsgType {
    Init = 0,
    SndRcv,
    SndInfo,
    RcvInfo,
    NxtInfo,
    PrInfo,
    AuthInfo,
    DstAddrV4,
    DstAddrV6,
}

/// Constants related to `enm sctp_sstat_state`
#[repr(i32)]
#[derive(Debug, Clone, Default)]
pub enum ConnState {
    #[default]
    Empty = 0,
    Closed,
    CookieWait,
    CookieEchoed,
    Established,
    ShutdownPending,
    ShutdownSent,
    ShutdownReceived,
    ShutdownAckSent,
}

/// PeerAddress: Structure representing SCTP Peer Address.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PeerAddress {
    assoc_id: AssociationId,
    address: libc::sockaddr_storage,
    state: i32,
    cwnd: u32,
    srtt: u32,
    rto: u32,
    mtu: u32,
}

/// ConnStatus: Status of an SCTP Connection
#[repr(C)]
#[derive(Clone)]
pub struct ConnStatus {
    pub assoc_id: AssociationId,
    pub state: ConnState,
    pub rwnd: u32,
    pub unacked_data: u16,
    pub pending_data: u16,
    pub instreams: u16,
    pub outstreams: u16,
    pub fragmentation_pt: u32,
    pub peer_primary: PeerAddress,
}

pub(crate) mod internal;
