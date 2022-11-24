use super::create_socket_bind_and_listen;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

#[allow(unused)]
use sctp_rs::*;

#[test]
fn socket_init_params_set_ostreams_success() {
    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

    let result =
        listener.sctp_subscribe_event(SctpEvent::Association, SubscribeEventAssocId::Future);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let client_ostreams = 100;
    let client_istreams = 5;
    let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToMany);
    let result = client_socket.sctp_setup_init_params(client_ostreams, client_istreams, 0, 0);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let assoc_id = client_socket.sctp_connectx(&[bindaddr]);
    assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());

    let result = listener.sctp_recv();
    assert!(result.is_ok(), "{:#}", result.err().unwrap());

    let notification = result.unwrap();
    assert!(
        matches!(
            notification,
            SctpNotificationOrData::Notification(SctpNotification::AssociationChange(
                AssociationChange { .. }
            ))
        ),
        "{:#?}",
        notification
    );

    if let SctpNotificationOrData::Notification(SctpNotification::AssociationChange(
        AssociationChange {
            ib_streams,
            ob_streams,
            ..
        },
    )) = notification
    {
        assert!(
            ib_streams == client_ostreams,
            "client_ostreams: {}, ib_streams: {}",
            client_ostreams,
            ib_streams
        );
        assert!(
            ob_streams == client_istreams,
            "client_istreams: {}, ob_streams: {}",
            client_istreams,
            ob_streams
        );
    } else {
        assert!(false, "Should never come here!: {:#?}", notification);
    };
}

#[test]
fn socket_sctp_req_recv_info_success() {
    let one2one_socket = SctpSocket::new_v4(SocketToAssociation::OneToOne);
    let result = one2one_socket.sctp_request_rcvinfo(true);
    assert!(result.is_ok(), "{:?}", result.err().unwrap());

    let one2many_socket = SctpSocket::new_v4(SocketToAssociation::OneToMany);
    let result = one2many_socket.sctp_request_rcvinfo(true);
    assert!(result.is_ok(), "{:?}", result.err().unwrap());
}

#[test]
fn test_bind_success() {
    let sctp_socket = SctpSocket::new_v4(SocketToAssociation::OneToOne);
    let bindaddr = Ipv4Addr::UNSPECIFIED;

    let result = sctp_socket.bind(SocketAddr::new(IpAddr::V4(bindaddr), 0));
    assert!(result.is_ok(), "{:?}", result.err().unwrap());
}

#[test]
fn test_bindx_inaddr_any_add_success() {
    let sctp_socket = SctpSocket::new_v4(SocketToAssociation::OneToOne);
    let bindaddr = Ipv4Addr::UNSPECIFIED;

    let result =
        sctp_socket.sctp_bindx(&[SocketAddr::new(IpAddr::V4(bindaddr), 0)], BindxFlags::Add);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
}

#[test]
fn test_bindx_inaddr6_any_add_success() {
    let sctp_socket = SctpSocket::new_v6(SocketToAssociation::OneToOne);
    let bindaddr = Ipv6Addr::UNSPECIFIED;

    let result =
        sctp_socket.sctp_bindx(&[SocketAddr::new(IpAddr::V6(bindaddr), 0)], BindxFlags::Add);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
}

#[test]
fn test_bindx_inaddr_any_add_and_remove_failure() {
    let sctp_socket = SctpSocket::new_v6(SocketToAssociation::OneToOne);
    let bindaddr6_localhost = Ipv6Addr::LOCALHOST;

    let result = sctp_socket.sctp_bindx(
        &[SocketAddr::new(IpAddr::V6(bindaddr6_localhost), 8080)],
        BindxFlags::Add,
    );
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let result = sctp_socket.sctp_bindx(
        &[SocketAddr::new(IpAddr::V6(bindaddr6_localhost), 8080)],
        BindxFlags::Remove,
    );
    assert!(result.is_err(), "{:#?}", result.ok().unwrap());
}
