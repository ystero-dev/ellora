#[cfg(test)]
mod tests {
    use sctp_rs::*;
    use std::net::SocketAddr;
    use std::sync::atomic::{AtomicU16, Ordering};

    static TEST_PORT_NO: AtomicU16 = AtomicU16::new(8080);

    fn create_socket_bind_and_listen(
        association: SocketToAssociation,
        v4: bool,
    ) -> (SctpListener, SocketAddr) {
        let sctp_socket = if v4 {
            SctpSocket::new_v4(association)
        } else {
            SctpSocket::new_v6(association)
        };
        let port = TEST_PORT_NO.fetch_add(1, Ordering::SeqCst);
        let bindaddr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

        let result = sctp_socket.bind(bindaddr);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());

        let listener = sctp_socket.listen(10);
        assert!(listener.is_ok(), "{:#?}", listener.err().unwrap());

        (listener.unwrap(), bindaddr)
    }

    #[test]
    fn listening_sctp_bindx_add_success() {
        let (listener, bindaddr) =
            create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

        let bindx_bindaddr: SocketAddr = format!("127.0.0.53:{}", bindaddr.port()).parse().unwrap();
        let result = listener.sctp_bindx(&[bindx_bindaddr], BindxFlags::Add);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    }

    #[test]
    fn listening_socket_no_connect_peeloff_failure() {
        let (listener, _) = create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

        let result = listener.sctp_peeloff(42);
        assert!(result.is_err(), "{:#?}", result.ok().unwrap());
    }

    #[test]
    fn listening_socket_one2one_connected_peeloff_failure() {
        let (listener, bindaddr) =
            create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

        let result =
            listener.sctp_subscribe_event(SctpEvent::Association, SubscribeEventAssocId::Future);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());

        let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToOne);
        let assoc_id = client_socket.sctp_connectx(&[bindaddr]);
        assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());

        let received = listener.sctp_peeloff(0);
        assert!(received.is_err(), "{:#?}", received.ok().unwrap());
    }

    #[test]
    fn listening_socket_one2many_connected_peeloff_success() {
        let (listener, bindaddr) =
            create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

        let result =
            listener.sctp_subscribe_event(SctpEvent::Association, SubscribeEventAssocId::Future);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());

        let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToMany);
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
                assoc_id, state, ..
            },
        )) = notification
        {
            let received = listener.sctp_peeloff(assoc_id);
            assert!(received.is_ok(), "{:#?}", received.err().unwrap());
            assert!(
                state == SctpAssocChangeState::SctpCommUp as u16,
                "{}",
                state
            );
        } else {
            assert!(false, "Should never come here!: {:#?}", notification);
        };
    }

    #[test]
    fn listening_one_2_one_listen_accept_success() {
        let (listener, bindaddr) =
            create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

        let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToOne);
        let assoc_id = client_socket.sctp_connectx(&[bindaddr]);
        assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());

        let accept = listener.accept();
        assert!(accept.is_ok(), "{:#?}", accept.err().unwrap());

        // Get Peer Address
        let (accepted, _address) = accept.unwrap();
        let result = accepted.sctp_getpaddrs(0);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    }

    #[test]
    fn listening_one_2_many_listen_accept_failure() {
        let (listener, bindaddr) =
            create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

        let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToMany);
        let assoc_id = client_socket.sctp_connectx(&[bindaddr]);
        assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());

        let accept = listener.accept();
        assert!(accept.is_err(), "{:#?}", accept.ok().unwrap());
    }

    #[test]
    fn socket_init_params_set_ostreams_success() {
        let (listener, bindaddr) =
            create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

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
}
