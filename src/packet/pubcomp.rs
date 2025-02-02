//! PUBCOMP

use std::io::{Read, Write};

use {Decodable, Encodable};
use control::{ControlType, FixedHeader, PacketType};
use control::variable_header::PacketIdentifier;
use packet::{Packet, PacketError};

/// `PUBCOMP` packet
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PubcompPacket {
    fixed_header: FixedHeader,
    packet_identifier: PacketIdentifier,
    payload: (),
}

impl PubcompPacket {
    pub fn new(pkid: u16) -> PubcompPacket {
        PubcompPacket {
            fixed_header: FixedHeader::new(PacketType::with_default(ControlType::PublishComplete), 2),
            packet_identifier: PacketIdentifier(pkid),
            payload: (),
        }
    }

    pub fn packet_identifier(&self) -> u16 {
        self.packet_identifier.0
    }

    pub fn set_packet_identifier(&mut self, pkid: u16) {
        self.packet_identifier.0 = pkid;
    }
}

impl Packet for PubcompPacket {
    type Payload = ();

    fn fixed_header(&self) -> &FixedHeader {
        &self.fixed_header
    }

    fn payload(self) -> Self::Payload {
        self.payload
    }

    fn payload_ref(&self) -> &Self::Payload {
        &self.payload
    }

    fn encode_variable_headers<W: Write>(&self, writer: &mut W) -> Result<(), PacketError<Self>> {
        self.packet_identifier.encode(writer)?;

        Ok(())
    }

    fn encoded_variable_headers_length(&self) -> u32 {
        self.packet_identifier.encoded_length()
    }

    fn decode_packet<R: Read>(reader: &mut R, fixed_header: FixedHeader) -> Result<Self, PacketError<Self>> {
        let packet_identifier: PacketIdentifier = PacketIdentifier::decode(reader)?;
        Ok(PubcompPacket {
               fixed_header: fixed_header,
               packet_identifier: packet_identifier,
               payload: (),
           })
    }
}
