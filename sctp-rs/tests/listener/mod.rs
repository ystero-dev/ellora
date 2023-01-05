use crate::{create_client_socket, create_socket_bind_and_listen};
use sctp_rs::*;
use std::net::SocketAddr;

// Tests for `accept` API for Listening Socket.
#[tokio::test]
async fn listening_one_2_one_listen_accept_success() {
    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

    let client_socket = create_client_socket(SocketToAssociation::OneToOne, true);

    let assoc_id = client_socket.sctp_connectx(&[bindaddr]).await;
    assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());

    let accept = listener.accept().await;
    assert!(accept.is_ok(), "{:#?}", accept.err().unwrap());

    // Get Peer Address
    let (accepted, _address) = accept.unwrap();
    let result = accepted.sctp_getpaddrs(0);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
}

#[tokio::test]
async fn listening_one_2_many_listen_accept_failure() {
    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

    let client_socket = create_client_socket(SocketToAssociation::OneToMany, true);

    let assoc_id = client_socket.sctp_connectx(&[bindaddr]).await;
    assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());

    let accept = listener.accept().await;
    assert!(accept.is_err(), "{:#?}", accept.ok().unwrap());
}

// Tests for `shutdown` API for Listening Socket.
// TODO:

// Test for `sctp_bindx` API for Listening Socket.
#[tokio::test]
async fn listening_sctp_bindx_add_success() {
    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

    let bindx_bindaddr: SocketAddr = format!("127.0.0.53:{}", bindaddr.port()).parse().unwrap();
    let result = listener.sctp_bindx(&[bindx_bindaddr], BindxFlags::Add);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
}

// Tests for `sctp_peeloff` API for Listening Socket.
#[tokio::test]
async fn listening_socket_no_connect_peeloff_failure() {
    let (listener, _) = create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

    let result = listener.sctp_peeloff(42);
    assert!(result.is_err(), "{:#?}", result.ok().unwrap());
}

#[tokio::test]
async fn listening_socket_one2one_connected_peeloff_failure() {
    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

    let result =
        listener.sctp_subscribe_events(&[Event::Association], SubscribeEventAssocId::Future);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let client_socket = create_client_socket(SocketToAssociation::OneToOne, true);

    let assoc_id = client_socket.sctp_connectx(&[bindaddr]).await;
    assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());

    let received = listener.sctp_peeloff(0);
    assert!(received.is_err(), "{:#?}", received.ok().unwrap());
}

#[tokio::test]
async fn listening_socket_one2many_connected_peeloff_success() {
    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

    let result =
        listener.sctp_subscribe_events(&[Event::Association], SubscribeEventAssocId::Future);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let client_socket = create_client_socket(SocketToAssociation::OneToMany, true);

    let assoc_id = client_socket.sctp_connectx(&[bindaddr]).await;
    assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());

    let result = listener.sctp_recv().await;
    assert!(result.is_ok(), "{:#}", result.err().unwrap());

    let notification = result.unwrap();
    assert!(
        matches!(
            notification,
            NotificationOrData::Notification(Notification::AssociationChange(
                AssociationChange { .. }
            ))
        ),
        "{:#?}",
        notification
    );

    if let NotificationOrData::Notification(Notification::AssociationChange(AssociationChange {
        assoc_id,
        state,
        ..
    })) = notification
    {
        let received = listener.sctp_peeloff(assoc_id);
        assert!(received.is_ok(), "{:#?}", received.err().unwrap());
        assert!(state == AssocChangeState::CommUp, "{:#?}", state);
    } else {
        assert!(false, "Should never come here!: {:#?}", notification);
    };
}

// Tests for `sctp_getpaddrs` for Listening Socket.
// TODO:

// Tests for `sctp_getladdrs` for Listening Socket.
// TODO:

// Tests for `sctp_recv` for Listening Socket.
// TODO:

// Tests for `sctp_send for Listening Socket.
// TODO:

// Tests for `sctp_subscribe_event`/`sctp_unsubscribe_event` for Listening Socket.
// TODO:
