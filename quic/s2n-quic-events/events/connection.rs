// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[event("transport::version_information")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.3.1
//# QUIC endpoints each have their own list of of QUIC versions they
//# support.
/// QUIC version
struct VersionInformation<'a> {
    server_versions: &'a [u32],
    client_versions: &'a [u32],
    chosen_version: Option<u32>,
}

#[event("transport:alpn_information")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.3.2
//# QUIC implementations each have their own list of application level
//# protocols and versions thereof they support.
/// Application level protocol
struct AlpnInformation<'a> {
    server_alpns: &'a [&'a [u8]],
    client_alpns: &'a [&'a [u8]],
    chosen_alpn: &'a [u8],
}

#[event("transport:packet_sent")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.3.5
/// Packet was sent
struct PacketSent {
    packet_header: PacketHeader,
}

#[event("transport:packet_received")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.3.6
/// Packet was received
struct PacketReceived {
    packet_header: PacketHeader,
}

#[event("connectivity:active_path_updated")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.1.8
/// Active path was updated
struct ActivePathUpdated<'a> {
    // TODO: many events seem to require PacketHeader. Make it more ergonomic
    // to include this field.
    // packet_header: PacketHeader,
    previous: Path<'a>,
    active: Path<'a>,
}

#[event("transport:path_created")]
/// A new path was created
struct PathCreated<'a> {
    active: Path<'a>,
    new: Path<'a>,
}

#[event("transport:frame_sent")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.3.5
// This diverges a bit from the qlog spec, which prefers to log data as part of the
// packet events.
/// Frame was sent
struct FrameSent {
    packet_header: PacketHeader,
    path_id: u64,
    frame: Frame,
}

#[event("transport:frame_received")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.3.6
// This diverges a bit from the qlog spec, which prefers to log data as part of the
// packet events.
/// Frame was received
struct FrameReceived {
    packet_header: PacketHeader,
    path_id: u64,
    frame: Frame,
}

#[event("recovery:packet_lost")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.4.5
/// Packet was lost
struct PacketLost<'a> {
    packet_header: PacketHeader,
    path: Path<'a>,
    bytes_lost: u16,
    is_mtu_probe: bool,
}

#[event("recovery:metrics_updated")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.4.2
/// Recovery metrics updated
struct RecoveryMetrics {
    path_id: u64,
    min_rtt: Duration,
    smoothed_rtt: Duration,
    latest_rtt: Duration,
    rtt_variance: Duration,
    max_ack_delay: Duration,
    pto_count: u32,
    congestion_window: u32,
    bytes_in_flight: u32,
}

#[event("security:key_update")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.2.1
/// Crypto key updated
struct KeyUpdate {
    key_type: KeyType,
}

#[event("connectivity:connection_started")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.1.2
/// Connection started
struct ConnectionStarted<'a> {
    path: Path<'a>,
}

#[event("connectivity:connection_closed")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.1.3
/// Connection closed
struct ConnectionClosed {
    error: crate::connection::Error,
}

#[event("transport:duplicate_packet")]
/// Duplicate packet received
struct DuplicatePacket {
    packet_header: PacketHeader,
    path_id: u64,
    error: DuplicatePacketError,
}

#[event("transport:datagram_sent")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.3.10
/// Datagram sent
struct DatagramSent {
    len: u16,
}

#[event("transport:datagram_received")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.3.11
/// Datagram received
struct DatagramReceived {
    len: u16,
}

#[event("transport:datagram_dropped")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.3.12
/// Datagram dropped
struct DatagramDropped {
    len: u16,
    reason: DropReason,
}

#[event("connectivity:connection_id_updated")]
//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#5.1.4
/// ConnectionId updated
struct ConnectionIdUpdated<'a> {
    path_id: u64,
    /// The endpoint that updated its connection id
    #[builder(crate::endpoint::Location)]
    cid_consumer: Location,
    previous: ConnectionId<'a>,
    current: ConnectionId<'a>,
}
