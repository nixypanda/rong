use std::{
    net::IpAddr,
    thread,
    time::{Duration, Instant},
};

use pnet::{
    packet::{
        icmp::{echo_reply::EchoReplyPacket, echo_request::MutableEchoRequestPacket, IcmpTypes},
        ip::IpNextHeaderProtocols,
        util, Packet,
    },
    transport::{self, transport_channel, TransportChannelType, TransportProtocol},
};
use structopt::StructOpt;

static DEFAULT_DATA_SIZE: u8 = 56;
static DEFAULT_HEADER_SIZE: u8 = 8;
static DEFAULT_BUFFER_SIZE: usize = 1024;
static INITIAL_SEQUENCE_NUMBER: u16 = 1;
static NUMBER_OF_PACKETS_TO_SEND: u16 = 5;

#[derive(Debug, StructOpt)]
#[structopt(name = "rong", about = "Rust toy clone of ping")]
struct CliOptions {
    #[structopt(short, long)]
    url: String,
}

fn main() -> anyhow::Result<()> {
    let cli_options = CliOptions::from_args();
    let packet_size = DEFAULT_DATA_SIZE + DEFAULT_HEADER_SIZE;

    let ping_destination_ip = cli_options.url.parse::<IpAddr>()?;

    println!(
        "PING {} ({:?}) {} bytes of data",
        cli_options.url, ping_destination_ip, DEFAULT_DATA_SIZE
    );

    let channel_type =
        TransportChannelType::Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx, mut rx) = transport_channel(DEFAULT_BUFFER_SIZE, channel_type)?;
    let mut response_iter = transport::icmp_packet_iter(&mut rx);

    let mut echo_packet_data = vec![0; packet_size.into()];

    let mut response_data = vec![];

    for sequence_number in
        INITIAL_SEQUENCE_NUMBER..(NUMBER_OF_PACKETS_TO_SEND + INITIAL_SEQUENCE_NUMBER)
    {
        let start_time = Instant::now();

        let echo_request = create_echo_request(&mut echo_packet_data, sequence_number)?;
        tx.send_to(echo_request, ping_destination_ip)?;
        let (response_packet, _ip_addr) = response_iter.next()?;

        let echo_reply = EchoReplyPacket::new(response_packet.packet())
            .ok_or_else(|| anyhow::anyhow!("Invalid ICMP Echo Reply"))?;

        // Validate if the reply was for the packet we actually sent out

        let elapsed_time = start_time.elapsed().as_millis();

        println!(
            "{} bytes from {}: icmp_seq={} time={}ms",
            packet_size,
            ping_destination_ip,
            echo_reply.get_sequence_number(),
            elapsed_time
        );
        // NOTE: Handle cases where packet is lost
        response_data.push(Some(elapsed_time));

        thread::sleep(Duration::from_secs(1));
    }

    println!("--- {} ping statistics ---", ping_destination_ip);
    print_stats(response_data);

    Ok(())
}

fn create_echo_request(
    echo_packet_data: &'_ mut [u8],
    sequence_number: u16,
) -> anyhow::Result<MutableEchoRequestPacket<'_>> {
    let mut echo_packet = MutableEchoRequestPacket::new(echo_packet_data)
        .ok_or_else(|| anyhow::anyhow!("Buffer is less than mininum required packet size"))?;
    echo_packet.set_sequence_number(sequence_number);
    echo_packet.set_icmp_type(IcmpTypes::EchoRequest);
    // Why this magic number?
    // Calculates a checksum. Used by ipv4 and icmp. The two bytes starting at `skipword * 2` will
    // be ignored. Supposed to be the checksum field, which is regarded as zero during calculation.
    let csum = util::checksum(echo_packet.packet(), 1);
    echo_packet.set_checksum(csum);

    Ok(echo_packet)
}

fn print_stats(responses: Vec<Option<u128>>) {
    let number_of_packets = responses.len();
    let successful_packets = responses.into_iter().flatten().collect::<Vec<u128>>();
    let loss_percent = (number_of_packets - successful_packets.len()) / number_of_packets * 100;

    let max = successful_packets.iter().max().unwrap_or(&0);
    let min = successful_packets.iter().min().unwrap_or(&0);
    let avg: f32 = successful_packets.iter().sum::<u128>() as f32 / number_of_packets as f32;

    println!(
        "{} packets transmitted, {} recieved, {}% packet loss",
        number_of_packets,
        successful_packets.len(),
        loss_percent
    );

    println!("rtt min/avg/max = {}/{}/{} ms", min, avg, max);
}
