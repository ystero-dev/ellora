use sctp_rs::*;

use crate::{create_client_socket, create_socket_bind_and_listen};

#[tokio::test]
async fn bindx_not_supported() {
    let connected = ConnectedSocket::from_rawfd(100);
    assert!(connected.is_err(), "{:?}", connected.ok().unwrap());

    // TODO: Write real test
}

#[tokio::test]
async fn connected_default_sendinfo_success() {
    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

    let client_socket = create_client_socket(SocketToAssociation::OneToOne, true);
    let result = client_socket.sctp_request_rcvinfo(true);
    assert!(result.is_ok(), "{:?}", result.err().unwrap());

    let result = client_socket.sctp_connectx(&[bindaddr]).await;
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let (connected, assoc_id) = result.unwrap();

    let accept = listener.accept().await;
    assert!(accept.is_ok(), "{:#?}", accept.err().unwrap());

    // Get Peer Address
    let (accepted, _client_addr) = accept.unwrap();

    let sid = 5;
    let ppid = 0x1234;
    let sendinfo = SendInfo {
        sid,
        ppid,
        flags: 1,
        assoc_id: 0,
        context: 0,
    };

    let result = accepted.sctp_set_default_sendinfo(sendinfo);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let senddata = SendData {
        payload: b"hello world!".to_vec(),
        snd_info: None,
    };
    let result = accepted.sctp_send(senddata.clone()).await;
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let result = connected.sctp_recv().await;
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let data = result.unwrap();
    assert!(
        matches!(data, NotificationOrData::Data(ReceivedData { .. })),
        "{:#?}",
        data
    );

    if let NotificationOrData::Data(ReceivedData {
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
        assert_eq!(
            rcv_info.sid, sid,
            "rcv_info.sid: {}, sid: {}",
            rcv_info.sid, sid
        );
        assert_eq!(
            rcv_info.ppid, ppid,
            "rcv_info.ppid: {:x}, ppid: {:x}",
            rcv_info.ppid, ppid
        );
        assert!(nxt_info.is_none(), "{:#?}", nxt_info.unwrap());
    } else {
        assert!(false, "Should never come here!: {:#?}", data);
    };
}

#[tokio::test]
async fn test_shutdown_event() {
    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

    let client_socket = create_client_socket(SocketToAssociation::OneToOne, true);
    let result = client_socket.sctp_request_rcvinfo(true);
    assert!(result.is_ok(), "{:?}", result.err().unwrap());

    let result = client_socket.sctp_connectx(&[bindaddr]).await;
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let (connected, client_assoc_id) = result.unwrap();
    let result = connected.sctp_subscribe_events(&[Event::Shutdown], SubscribeEventAssocId::All);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let accept = listener.accept().await;
    assert!(accept.is_ok(), "{:#?}", accept.err().unwrap());
    let (accepted, _) = accept.unwrap();

    // drop the connected socket, so that should generate shutdown event.
    drop(accepted);

    let result = connected.sctp_recv().await;
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let data = result.unwrap();
    assert!(
        matches!(
            data,
            NotificationOrData::Notification(Notification::Shutdown(Shutdown { .. }))
        ),
        "{:#?}",
        data
    );

    if let NotificationOrData::Notification(Notification::Shutdown(Shutdown { assoc_id, .. })) =
        data
    {
        assert_eq!(
            client_assoc_id, assoc_id,
            "client_assoc_id: {}, Event assoc_id: {}",
            client_assoc_id, assoc_id
        );
    } else {
        assert!(false, "Should never come here!: {:#?}", data);
    }
}

#[tokio::test]
async fn test_get_status() {
    let (listener, bindaddr) = create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

    let client_socket = create_client_socket(SocketToAssociation::OneToOne, true);
    let result = client_socket.sctp_request_rcvinfo(true);
    assert!(result.is_ok(), "{:?}", result.err().unwrap());

    let result = client_socket.sctp_connectx(&[bindaddr]).await;
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let (connected, client_assoc_id) = result.unwrap();

    let accept = listener.accept().await;
    assert!(accept.is_ok(), "{:#?}", accept.err().unwrap());
    let (accepted, client_addr) = accept.unwrap();

    let result = connected.sctp_get_status(0);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let status = result.unwrap();
    assert_eq!(
        status.assoc_id, client_assoc_id,
        "client assoc ID: {}, status assoc ID: {}",
        client_assoc_id, status.assoc_id
    );

    let result = accepted.sctp_get_status(0);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    let status = result.unwrap();
    assert_eq!(
        client_addr, status.peer_primary.address,
        "Client Addres: {}, Peer Primary Address: {}",
        client_addr, status.peer_primary.address
    );
}
