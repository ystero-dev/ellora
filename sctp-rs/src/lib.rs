//! SCTP Implementation for Rust 'async' runtimes.
//!
//! The Goal of this implementation is being able to use SCTP within Rust's async ecosystem. Also,
//! this implementation is a Pure Rust implementation leveraging the kernel SCTP stack in the Linux
//! kernel without requiring `libsctp` or similar libraries to be present and makes use of the
//! `libc` crate for system calls. This crate implements [Socket API
//! Extensions](https://datatracker.ietf.org/doc/html/rfc6458), that are expected in a modern
//! implementation of SCTP stack. As a result, APIs that are marked as `DEPRECATED` in the RFC will
//! not will not be implemented initially.
//!
//! Also, the APIs are designed such that they are idiomatic Rust APIs and making use of
//! appropriate types thus making use of the [`std::net::SocketAddr`] structures wherever
//! appropriate rather than using the [`libc::sockaddr`] structures for example.
//!
//! # Example
//!
//! The examples below will help you to get started using the APIs in your application.
//!
//! ```rust,no_run
//!
//! # #[tokio::main(flavor="current_thread")]
//! # async fn main() -> std::io::Result<()> {
//!
//! // Create a TCP Style (Socket-to-association is 1-1) socket.
//! let client = sctp_rs::Socket::new_v4(sctp_rs::SocketToAssociation::OneToOne)?;
//!
//! let bind_addr: std::net::SocketAddr = "127.0.0.1:8080".parse().unwrap();
//! client.bind(bind_addr)?;
//!
//! // Listen on the socket listen queue size of 10. Normally this number should be considerably
//! // higher like 100 or so.
//! let listener = client.listen(10)?;
//!
//! // Accept on the socket and process data.
//! let (accepted, _client_addr) = listener.accept().await?;
//!
//! loop {
//!     let notification_or_data = accepted.sctp_recv().await?;
//!     match notification_or_data {
//!         sctp_rs::NotificationOrData::Notification(n) => {
//!             // Process Notification
//!         },
//!         sctp_rs::NotificationOrData::Data(n) => {
//!             // Process Data
//!         }
//!     }
//! }
//!
//! # Ok(())
//! # }

mod connected_socket;
mod listener;
mod socket;

#[doc(inline)]
pub use socket::Socket;

#[doc(inline)]
pub use listener::Listener;

#[doc(inline)]
pub use connected_socket::ConnectedSocket;

mod internal;

mod consts;

mod types;

#[doc(inline)]
pub use types::{
    AssocChangeState, AssociationChange, AssociationId, BindxFlags, CmsgType, ConnStatus, Event,
    Notification, NotificationOrData, NxtInfo, RcvInfo, ReceivedData, SendData, SendInfo,
    SocketToAssociation, SubscribeEventAssocId,
};
