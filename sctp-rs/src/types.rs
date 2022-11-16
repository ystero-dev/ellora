//! Types used by the Public APIs

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

/// SctpNotificationOrData: A type returned by a `sctp_recvv` call.
#[derive(Debug)]
pub enum SctpNotificationOrData {
    /// SCTP Notification received by an `sctp_recvv` call
    Notification(SctpNotification),

    /// SCTP Data Received by an association.
    Data(SctpReceivedData),
}

/// Structure Representing SCTP Received Data.
///
/// This structure is returned by the `sctp_recv` API call. This contains in addition to 'received'
/// data, any ancillary data that is received during the underlying system call.
#[derive(Debug)]
pub struct SctpReceivedData {
    pub data: Vec<u8>,
    pub rcv_info: Option<SctpRcvInfo>,
    pub nxt_info: Option<SctpNxtInfo>,
}

/// Structure Representing Ancillary Receive Information (See Section 5.3.5 of RFC 6458)
#[derive(Debug)]
pub struct SctpRcvInfo {
    sid: u16,
    ssn: u16,
    flags: u16,
    ppid: u32,
    tsn: u32,
    cumtsn: u32,
    context: u32,
    assoc_id: SctpAssociationId,
}

/// Structure representing Ancillary next information (See Section 5.3.5)
#[derive(Debug)]
pub struct SctpNxtInfo {
    sid: u16,
    flags: u16,
    ppid: u32,
    lenghtn: u32,
    assoc_id: SctpAssociationId,
}

#[derive(Debug)]
pub enum SctpNotification {
    /// Association Change Notification. See Section 6.1.1 of RFC 6458
    AssociationChange(AssociationChange),

    /// A Catchall Notification type for the Notifications that are not supported
    Unsupported,
}

/// AssociationChange: Structure returned as notification for Association Change.
///
/// To subscribe to this notification type, An application should call `sctp_subscribe_event` using
/// the [`SctpEvent`] type as [`SctpEvent::Association`].
#[repr(C)]
#[derive(Debug)]
pub struct AssociationChange {
    pub assoc_type: u16,
    pub flags: u16,
    pub length: u32,
    pub state: u16,
    pub error: u16,
    pub ob_streams: u16,
    pub ib_streams: u16,
    pub assoc_id: SctpAssociationId,
    pub info: Vec<u8>,
}

/// SctpEvent: Used for Subscribing for SCTP Events
///
/// See [`sctp_subscribe_events`][`crate::SctpListener::sctp_subscribe_event`] for the usage.
#[repr(u16)]
#[derive(Debug)]
pub enum SctpEvent {
    DataIo = (1 << 15),
    Association,
    Address,
    SendFailure,
    PeerError,
    Shutdown,
    PartialDelivery,
    AdaptationLayer,
    Authentication,
    SenderDry,
    StreamReset,
    AssociationReset,
    StreamChange,
    SendFailureEvent,
}

/// SubscribeEventAssocId: AssociationID Used for Event Subscription
///
/// Note: repr should be same as `SctpAssociationId` (ie. `i32`)
#[repr(i32)]
pub enum SubscribeEventAssocId {
    Future,
    Current,
    All,
    Value(SctpAssociationId),
}

impl From<SubscribeEventAssocId> for SctpAssociationId {
    fn from(value: SubscribeEventAssocId) -> Self {
        match value {
            SubscribeEventAssocId::Future => 0 as Self,
            SubscribeEventAssocId::Current => 1 as Self,
            SubscribeEventAssocId::All => 2 as Self,
            SubscribeEventAssocId::Value(v) => v,
        }
    }
}

pub(crate) mod internal;
