use std::fmt::Debug;

use futures::stream::StreamExt;
use netlink_packet_core::NetlinkPayload;
use netlink_packet_route::address::AddressMessage;
use netlink_packet_route::link::LinkMessage;
use netlink_packet_route::neighbour::NeighbourMessage;
use netlink_packet_route::neighbour_table::NeighbourTableMessage;
use netlink_packet_route::nsid::NsidMessage;
use netlink_packet_route::route::RouteMessage;
use netlink_packet_route::rule::RuleMessage;
use netlink_packet_route::tc::TcMessage;
use netlink_packet_route::RouteNetlinkMessage;
use netlink_sys::{AsyncSocket, SocketAddr};
use rtnetlink::{
    constants::{
        RTMGRP_IPV4_IFADDR, RTMGRP_IPV4_MROUTE, RTMGRP_IPV4_ROUTE,
        RTMGRP_IPV4_RULE, RTMGRP_IPV6_IFADDR, RTMGRP_IPV6_IFINFO,
        RTMGRP_IPV6_MROUTE, RTMGRP_IPV6_ROUTE, RTMGRP_LINK, RTMGRP_NEIGH,
        RTMGRP_NOTIFY, RTMGRP_TC,
    },
    new_connection,
};

trait OnCreate {
    fn on_create(&self)
    where
        Self: Debug,
    {
        println!("Created: {self:?}");
    }
}

trait OnDelete {
    fn on_delete(&self)
    where
        Self: Debug,
    {
        println!("Deleted: {self:?}");
    }
}

trait OnSet {
    fn on_set(&self)
    where
        Self: Debug,
    {
        println!("Setting: {self:?}");
    }
}

impl OnCreate for RouteMessage {}
impl OnCreate for LinkMessage {}
impl OnCreate for AddressMessage {}
impl OnCreate for NeighbourMessage {}
impl OnCreate for NeighbourTableMessage {}
impl OnCreate for TcMessage {}
impl OnCreate for NsidMessage {}
impl OnCreate for RuleMessage {}

impl OnDelete for RouteMessage {}
impl OnDelete for LinkMessage {}
impl OnDelete for AddressMessage {}
impl OnDelete for NeighbourMessage {}
impl OnDelete for TcMessage {}
impl OnDelete for NsidMessage {}
impl OnDelete for RuleMessage {}

impl OnSet for LinkMessage {}
impl OnSet for NeighbourTableMessage {}

#[tokio::main]
async fn main() -> Result<(), String> {
    let (mut connection, _, mut messages) =
        new_connection().map_err(|e| format!("{e}"))?;

    let mgroup_flags = RTMGRP_LINK
        | RTMGRP_NOTIFY
        | RTMGRP_NEIGH
        | RTMGRP_TC
        | RTMGRP_IPV4_IFADDR
        | RTMGRP_IPV4_MROUTE
        | RTMGRP_IPV4_ROUTE
        | RTMGRP_IPV4_RULE
        | RTMGRP_IPV6_IFADDR
        | RTMGRP_IPV6_MROUTE
        | RTMGRP_IPV6_ROUTE
        | RTMGRP_IPV6_IFINFO;

    let addr = SocketAddr::new(0, mgroup_flags);
    connection
        .socket_mut()
        .socket_mut()
        .bind(&addr)
        .expect("failed to bind");
    tokio::spawn(connection);

    while let Some((message, _)) = messages.next().await {
        let payload = &message.payload;
        if let NetlinkPayload::InnerMessage(message) = payload {
            match message {
                RouteNetlinkMessage::NewRoute(m) => m.on_create(),
                RouteNetlinkMessage::DelRoute(m) => m.on_delete(),
                // RouteNetlinkMessage::GetRoute(_) => todo!(),
                RouteNetlinkMessage::NewLink(m) => m.on_create(),
                RouteNetlinkMessage::DelLink(m) => m.on_delete(),
                RouteNetlinkMessage::SetLink(m) => m.on_set(),
                // RouteNetlinkMessage::GetLink(_) => todo!(),
                RouteNetlinkMessage::NewLinkProp(m) => m.on_create(),
                RouteNetlinkMessage::DelLinkProp(m) => m.on_delete(),
                RouteNetlinkMessage::NewAddress(m) => m.on_create(),
                RouteNetlinkMessage::DelAddress(m) => m.on_delete(),
                // RouteNetlinkMessage::GetAddress(_) => todo!(),
                RouteNetlinkMessage::NewNeighbour(m) => m.on_create(),
                RouteNetlinkMessage::DelNeighbour(m) => m.on_delete(),
                // RouteNetlinkMessage::GetNeighbour(_) => todo!(),
                RouteNetlinkMessage::NewNeighbourTable(m) => m.on_create(),
                RouteNetlinkMessage::SetNeighbourTable(m) => m.on_set(),
                // RouteNetlinkMessage::GetNeighbourTable(_) => todo!(),
                RouteNetlinkMessage::NewQueueDiscipline(m) => m.on_create(),
                RouteNetlinkMessage::DelQueueDiscipline(m) => m.on_delete(),
                // RouteNetlinkMessage::GetQueueDiscipline(_) => todo!(),
                RouteNetlinkMessage::NewTrafficClass(m) => m.on_create(),
                RouteNetlinkMessage::DelTrafficClass(m) => m.on_delete(),
                // RouteNetlinkMessage::GetTrafficClass(_) => todo!(),
                RouteNetlinkMessage::NewTrafficFilter(m) => m.on_create(),
                RouteNetlinkMessage::DelTrafficFilter(m) => m.on_delete(),
                // RouteNetlinkMessage::GetTrafficFilter(_) => todo!(),
                RouteNetlinkMessage::NewTrafficChain(m) => m.on_create(),
                RouteNetlinkMessage::DelTrafficChain(m) => m.on_delete(),
                // RouteNetlinkMessage::GetTrafficChain(_) => todo!(),
                RouteNetlinkMessage::NewNsId(m) => m.on_create(),
                RouteNetlinkMessage::DelNsId(m) => m.on_delete(),
                // RouteNetlinkMessage::GetNsId(_) => todo!(),
                RouteNetlinkMessage::NewRule(m) => m.on_create(),
                RouteNetlinkMessage::DelRule(m) => m.on_delete(),
                // RouteNetlinkMessage::GetRule(_) => todo!(),
                _ => {}
            }
        }
    }
    Ok(())
}
