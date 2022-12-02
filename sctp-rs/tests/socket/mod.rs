use super::create_socket_bind_and_listen;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

#[allow(unused)]
use sctp_rs::*;

#[tokio::test]
async fn socket_connect_basic_send_recv_req_info_on_and_off() {
    let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToMany);
    let result =
        client_socket.sctp_subscribe_event(SctpEvent::Association, SubscribeEventAssocId::Current);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    // Request Receive Info on client socket
    let result = client_socket.sctp_request_rcvinfo(true);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

    let sock_and_assoc_id = client_socket.sctp_connectx(&[bindaddr]).await;
    assert!(
        sock_and_assoc_id.is_ok(),
        "{:#?}",
        sock_and_assoc_id.err().unwrap()
    );
    let (connected, assoc_id) = sock_and_assoc_id.unwrap();
    eprintln!("assoc_id: {}", assoc_id);

    let laddrs = connected.sctp_getladdrs(assoc_id);
    assert!(laddrs.is_ok(), "{:#?}", laddrs.err().unwrap());

    let client_addr = laddrs.unwrap()[0];

    let senddata = SctpSendData {
        payload: b"hello world!".to_vec(),
        snd_info: None,
    };
    let result = listener.sctp_send(client_addr, senddata.clone());
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let result = connected.sctp_recv();
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let data = result.unwrap();
    assert!(
        matches!(data, SctpNotificationOrData::Data(SctpReceivedData { .. })),
        "{:#?}",
        data
    );

    if let SctpNotificationOrData::Data(SctpReceivedData {
        payload,
        rcv_info,
        nxt_info,
    }) = data
    {
        assert!(
            payload == b"hello world!".to_vec(),
            "received_payload: {:?}",
            payload,
        );
        assert!(rcv_info.is_some());
        let rcv_info = rcv_info.unwrap();
        assert_eq!(
            rcv_info.assoc_id, assoc_id,
            "rcv_info.assoc_id: {}, assoc_id: {}",
            rcv_info.assoc_id, assoc_id
        );
        assert!(nxt_info.is_none(), "{:#?}", nxt_info.unwrap());
    } else {
        assert!(false, "Should never come here!: {:#?}", data);
    };

    // Now turn off Request Receive Info on client socket
    let result = connected.sctp_request_rcvinfo(false);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    // Again send the data to client
    let result = listener.sctp_send(client_addr, senddata);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let result = connected.sctp_recv();
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let data = result.unwrap();
    assert!(
        matches!(data, SctpNotificationOrData::Data(SctpReceivedData { .. })),
        "{:#?}",
        data
    );

    if let SctpNotificationOrData::Data(SctpReceivedData {
        payload,
        rcv_info,
        nxt_info,
    }) = data
    {
        assert!(
            payload == b"hello world!".to_vec(),
            "received_payload: {:?}",
            payload,
        );
        assert!(rcv_info.is_none(), "{:#?}", rcv_info.unwrap());
        assert!(nxt_info.is_none(), "{:#?}", nxt_info.unwrap());
    } else {
        assert!(false, "Should never come here!: {:#?}", data);
    };
}

#[tokio::test]
async fn socket_send_recv_nxtinfo_test() {
    let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToMany);
    let result =
        client_socket.sctp_subscribe_event(SctpEvent::Association, SubscribeEventAssocId::Current);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    // Request Receive Info on client socket
    let result = client_socket.sctp_request_nxtinfo(true);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

    let sock_and_assoc_id = client_socket.sctp_connectx(&[bindaddr]).await;
    assert!(
        sock_and_assoc_id.is_ok(),
        "{:#?}",
        sock_and_assoc_id.err().unwrap()
    );
    let (connected, assoc_id) = sock_and_assoc_id.unwrap();

    let laddrs = connected.sctp_getladdrs(assoc_id);
    assert!(laddrs.is_ok(), "{:#?}", laddrs.err().unwrap());

    let client_addr = laddrs.unwrap()[0];

    let senddata = SctpSendData {
        payload: b"hello world!".to_vec(),
        snd_info: None,
    };
    let result = listener.sctp_send(client_addr, senddata.clone());
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    // Send again
    let result = listener.sctp_send(client_addr, senddata.clone());
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    // First Receive nxtinfo should not be none.
    let result = connected.sctp_recv();
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let data = result.unwrap();
    assert!(
        matches!(data, SctpNotificationOrData::Data(SctpReceivedData { .. })),
        "{:#?}",
        data
    );

    if let SctpNotificationOrData::Data(SctpReceivedData {
        payload,
        rcv_info,
        nxt_info,
    }) = data
    {
        assert!(
            payload == b"hello world!".to_vec(),
            "received_payload: {:?}",
            payload,
        );
        assert!(rcv_info.is_none(), "{:#?}", rcv_info.unwrap());
        assert!(nxt_info.is_some());
    } else {
        assert!(false, "Should never come here!: {:#?}", data);
    };

    // First Receive nxtinfo should not be none.
    let result = connected.sctp_recv();
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let data = result.unwrap();
    assert!(
        matches!(data, SctpNotificationOrData::Data(SctpReceivedData { .. })),
        "{:#?}",
        data
    );

    if let SctpNotificationOrData::Data(SctpReceivedData {
        payload,
        rcv_info,
        nxt_info,
    }) = data
    {
        assert!(
            payload == b"hello world!".to_vec(),
            "received_payload: {:?}",
            payload,
        );
        assert!(rcv_info.is_none(), "{:#?}", rcv_info.unwrap());
        assert!(nxt_info.is_none(), "{:#?}", nxt_info.unwrap());
    } else {
        assert!(false, "Should never come here!: {:#?}", data);
    };
}

#[tokio::test]
async fn socket_init_params_set_ostreams_success() {
    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

    let result =
        listener.sctp_subscribe_event(SctpEvent::Association, SubscribeEventAssocId::Future);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let client_ostreams = 100;
    let client_istreams = 5;
    let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToMany);
    let result = client_socket.sctp_setup_init_params(client_ostreams, client_istreams, 0, 0);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let assoc_id = client_socket.sctp_connectx(&[bindaddr]).await;
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

#[tokio::test]
async fn socket_sctp_req_recv_info_success() {
    let one2one_socket = SctpSocket::new_v4(SocketToAssociation::OneToOne);
    let result = one2one_socket.sctp_request_rcvinfo(true);
    assert!(result.is_ok(), "{:?}", result.err().unwrap());

    let one2many_socket = SctpSocket::new_v4(SocketToAssociation::OneToMany);
    let result = one2many_socket.sctp_request_rcvinfo(true);
    assert!(result.is_ok(), "{:?}", result.err().unwrap());
}

#[tokio::test]
async fn test_bind_success() {
    let sctp_socket = SctpSocket::new_v4(SocketToAssociation::OneToOne);
    let bindaddr = Ipv4Addr::UNSPECIFIED;

    let result = sctp_socket.bind(SocketAddr::new(IpAddr::V4(bindaddr), 0));
    assert!(result.is_ok(), "{:?}", result.err().unwrap());
}

#[tokio::test]
async fn test_bindx_inaddr_any_add_success() {
    let sctp_socket = SctpSocket::new_v4(SocketToAssociation::OneToOne);
    let bindaddr = Ipv4Addr::UNSPECIFIED;

    let result =
        sctp_socket.sctp_bindx(&[SocketAddr::new(IpAddr::V4(bindaddr), 0)], BindxFlags::Add);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
}

#[tokio::test]
async fn test_bindx_inaddr6_any_add_success() {
    let sctp_socket = SctpSocket::new_v6(SocketToAssociation::OneToOne);
    let bindaddr = Ipv6Addr::UNSPECIFIED;

    let result =
        sctp_socket.sctp_bindx(&[SocketAddr::new(IpAddr::V6(bindaddr), 0)], BindxFlags::Add);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
}

#[tokio::test]
async fn test_bindx_inaddr_any_add_and_remove_failure() {
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

#[tokio::test]
async fn test_connect_no_listen_failure() {
    let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToMany);
    let connect_addr: SocketAddr = "127.0.0.53:8080".parse().unwrap();

    let result = client_socket.connect(connect_addr).await;
    assert!(result.is_err(), "{:?}", result.ok().unwrap());
    let err = result.err().unwrap();
    assert_eq!(err.raw_os_error(), Some(libc::ECONNREFUSED));
}
