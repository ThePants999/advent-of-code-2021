use itertools::Itertools;

pub fn day16(input_lines: &[String]) -> (u64, u64) {
    let input = Input::parse(&input_lines[0]);
    let packets = input.get_packets();

    let part1 = packets.iter().map(|packet| packet.version_sum()).sum::<u64>();
    let part2 = packets[0].value();
    (part1, part2)
}

struct Input {
    bytes: Vec<u8>,
    current_index: usize,
    current_offset: usize,
}

impl Input {
    fn parse(data: &str) -> Self {
        Self {
            bytes: data.chars().tuples().map(|(first, second)| (first.to_digit(16).expect("Invalid input") << 4) as u8 + second.to_digit(16).expect("Invalid input") as u8).collect(),
            current_index: 0,
            current_offset: 0,
        }
    }

    fn get_bits(&mut self, num_bits: usize) -> u64 {
        let first_byte = self.bytes[self.current_index] & (0b11111111 >> self.current_offset);
        if num_bits <= (8 - self.current_offset) {
            // Everything we want is in the first byte.
            let value = (first_byte >> (8 - self.current_offset - num_bits)) as u64;

            self.current_offset += num_bits;
            if self.current_offset == 8 {
                self.current_offset = 0;
                self.current_index += 1;
            }

            value
        } else {
            // We want everything from the first byte, and maybe then some.
            let mut value = first_byte as u64;
            let mut bits_remaining = num_bits - (8 - self.current_offset);
            while bits_remaining > 0 {
                self.current_index += 1;
                if bits_remaining > 8 {
                    // We want this entire byte, and it's not the last one.
                    value <<= 8;
                    value += self.bytes[self.current_index] as u64;
                    bits_remaining -= 8;
                } else {
                    // This is the last byte.
                    value <<= bits_remaining;
                    value += (self.bytes[self.current_index] as u64) >> (8 - bits_remaining);

                    if bits_remaining == 8 {
                        self.current_index += 1;
                        self.current_offset = 0;
                    } else {
                        self.current_offset = bits_remaining;
                    }

                    bits_remaining = 0;
                }
            }
            value
        }
    }

    fn get_packets(mut self) -> Vec<Packet> {
        let mut finished_packets: Vec<Packet> = Vec::new();
        let mut packets_under_construction: Vec<Packet> = Vec::new();

        // Once we're looking at the final byte and we've finished
        // parsing a packet, we're done - the final packet cannot fit
        // within a byte, since the last packet must be a literal value
        // and literals cannot fit within a single byte.
        while self.current_index < self.bytes.len() - 1 {
            let packet = self.parse_packet();
            Self::handle_packet(&mut finished_packets, &mut packets_under_construction, packet);
        }

        finished_packets
    }

    fn handle_packet(finished_packets: &mut Vec<Packet>, packets_under_construction: &mut Vec<Packet>, packet: Packet) {
        if packet.is_complete() {
            if let Some(mut parent_packet) = packets_under_construction.pop() {
                parent_packet.append_sub_packet(packet);
                Self::handle_packet(finished_packets, packets_under_construction, parent_packet);
            } else {
                finished_packets.push(packet);
            }
        } else {
            packets_under_construction.push(packet);
        }

    }

    fn parse_packet(&mut self) -> Packet {
        let version = self.get_bits(3);
        let packet_type = self.get_bits(3);

        if packet_type == 4 {
            // Literal value
            let mut value_length = 0usize;
            let mut value = 0u64;
            loop {
                value_length += 5;
                let group = self.get_bits(5);
                value <<= 4;
                value += group & 0b1111;
                if group & 0b10000 == 0 {
                    break;
                }
            }

            Packet { version, total_length: 6 + value_length, packet_type: PacketType::LiteralValue(value) }
        } else {
            // Operator
            let op_type = match packet_type {
                0 => OperatorPacketType::Sum,
                1 => OperatorPacketType::Product,
                2 => OperatorPacketType::Minimum,
                3 => OperatorPacketType::Maximum,
                5 => OperatorPacketType::GreaterThan,
                6 => OperatorPacketType::LessThan,
                7 => OperatorPacketType::EqualTo,
                _ => unreachable!("Invalid operator packet type"),
            };

            let length_type = self.get_bits(1);
            if length_type == 0 {
                // Total length of sub-packets
                let length = self.get_bits(15);
                let sub_packets = SubPackets::new(length as usize);
                Packet { version, total_length: 22, packet_type: PacketType::Operator(op_type, OperatorSubPacketType::Length, sub_packets) }
            } else {
                // Number of sub-packets
                let length = self.get_bits(11);
                let sub_packets = SubPackets::new(length as usize);
                Packet { version, total_length: 18, packet_type: PacketType::Operator(op_type, OperatorSubPacketType::Number, sub_packets) }
            }

        }
    }
}

struct Packet {
    version: u64,
    total_length: usize,
    packet_type: PacketType,
}

impl Packet {
    fn append_sub_packet(&mut self, sub_packet: Packet) {
        if let PacketType::Operator(_, _, sub_packets) = &mut self.packet_type {
            self.total_length += sub_packet.total_length;
            sub_packets.packets.push(sub_packet);
        } else {
            unreachable!("Sub-packet appended to literal value!");
        }
    }

    fn is_complete(&self) -> bool {
        match &self.packet_type {
            PacketType::LiteralValue(_) => true,
            PacketType::Operator(_, OperatorSubPacketType::Number, sub_packets) => sub_packets.packets.len() == sub_packets.length,
            PacketType::Operator(_, OperatorSubPacketType::Length, sub_packets) => sub_packets.packets.iter().map(|packet| packet.total_length).sum::<usize>() == sub_packets.length,
        }
    }

    fn version_sum(&self) -> u64 {
        match &self.packet_type {
            PacketType::LiteralValue(_) => self.version,
            PacketType::Operator(_, _, sub_packets) => self.version + sub_packets.packets.iter().map(|packet| packet.version_sum()).sum::<u64>()
        }
    }

    fn value(&self) -> u64 {
        match &self.packet_type {
            PacketType::LiteralValue(val) => *val,
            PacketType::Operator(op_type, _, sub_packets) => {
                let mut sub_values = sub_packets.packets.iter().map(|packet| packet.value());
                match op_type {
                    OperatorPacketType::Sum => sub_values.sum(),
                    OperatorPacketType::Product => sub_values.product(),
                    OperatorPacketType::Minimum => sub_values.min().unwrap(),
                    OperatorPacketType::Maximum => sub_values.max().unwrap(),
                    OperatorPacketType::GreaterThan => {
                        assert!(sub_packets.packets.len() == 2);
                        if sub_values.next().unwrap() > sub_values.next().unwrap() { 1 } else { 0 }
                    },
                    OperatorPacketType::LessThan => {
                        assert!(sub_packets.packets.len() == 2);
                        if sub_values.next().unwrap() < sub_values.next().unwrap() { 1 } else { 0 }
                    },
                    OperatorPacketType::EqualTo => {
                        assert!(sub_packets.packets.len() == 2);
                        if sub_values.next().unwrap() == sub_values.next().unwrap() { 1 } else { 0 }
                    },
                }
            }
        }
    }
}

enum PacketType {
    LiteralValue(u64),
    Operator(OperatorPacketType, OperatorSubPacketType, SubPackets),
}

enum OperatorPacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

enum OperatorSubPacketType {
    Number,
    Length,
}

struct SubPackets {
    length: usize,
    packets: Vec<Packet>,
}

impl SubPackets {
    fn new(length: usize) -> Self {
        Self {
            length,
            packets: Vec::new(),
        }
    }
}