use aoc_util::input_reader;


#[derive(Debug)]
enum PacketType {
    Literal(u64), // literal value
    Operator(u32), // operation?
}

#[derive(Debug)]
struct Packet {
    version: i32,
    kind: PacketType,
    length_type_id : Option<String>,
    end: usize
}

fn main() {
    let hex_data = input_reader::read_line_into_vec::<String>("input.txt", None);
    let bit_str = hex_to_bin(hex_data);
    //println!("\n{:?}", bit_str);
    let packets = parse_packets(&bit_str, None);
    let version_sum : i32 = packets.iter().map(|p| p.version).sum();
    println!("\npacket version sum: {}", version_sum);

}

fn parse_packets(bit_str : &str, n_packets: Option<usize>) -> Vec<Packet> {
    let mut p_start = 0;
    let mut p_found = 0;
    let mut packets = vec![];
    //println!("{}", bit_str.len());
    while !bit_str[p_start..].is_empty() {

        if n_packets.is_some() && p_found >= n_packets.unwrap() || bit_str[p_start..].chars().all(|c| c == '0') {
            break;
        }

        //println!("p_start: {}, n_packets: {}/{:?}, new bits: {}", p_start, p_found, n_packets, &bit_str[p_start..]);
        let (pv_bits, mut pv_str) = (&bit_str[p_start..p_start+3], String::from("0"));
        pv_str.push_str(pv_bits);
        let pv = i32::from_str_radix(&pv_str, 2).unwrap();

        let (pt_bits, mut pt_str) = (&bit_str[p_start+3..p_start+6], String::from("0"));
        pt_str.push_str(pt_bits);
        let pt = i32::from_str_radix(&pt_str, 2).unwrap();
        //println!("version: {:?}  type: {:?}", pv, pt);
  
        let mut more_packets : Vec<Packet> = match pt {
            4 => {
                let val = step_literal_packet(p_start, bit_str);
                let extra_signal_bits = val.len()/4;
                let mut p_end = p_start + 6 + val.len() + extra_signal_bits;
                //p_end += 4 - (p_end % 4); // zero padding
                let p = Packet {
                    version: pv,
                    kind: PacketType::Literal(u64::from_str_radix(&val, 2).unwrap()),
                    length_type_id: None,
                    end: p_end
                };
                vec![p]
            },
            _ => {
                let ltid : String = (&bit_str[p_start+6..p_start+7]).into();
                let len_bits = if ltid == "0" {15} else {11};
                let mut more_pkts = step_operator_packet(&ltid, &bit_str[p_start..]);
                let sub_pkt_end = more_pkts.iter().max_by_key(|p| p.end).unwrap().end;
                let this_p = Packet {
                    version: pv,
                    kind: PacketType::Operator(0),
                    length_type_id: Some(ltid),
                    end: p_start + sub_pkt_end + 7 + len_bits
                };
                //println!("{:?}", this_p);
                let mut packets = vec![this_p];
                packets.append(&mut more_pkts);
                packets
            },
        };
        p_start = more_packets.iter().max_by_key(|p| p.end).unwrap().end;   
        p_found += 1;
        //println!("p_start: {}, left: {}", p_start, &bit_str[p_start..]);
        packets.append(&mut more_packets); 
    }
    packets
}

fn step_literal_packet(start_i: usize, bit_str: &str) -> String {
    let mut stepper = start_i+6; // skip first 6 (version and type)
    let mut view = &bit_str[stepper..];
    let mut lit_bin_str = String::new();
    while &view[0..1] != "0" {
        lit_bin_str.push_str(&view[1..5]);
        stepper = stepper+5;
        view = &bit_str[stepper..];
    }
    lit_bin_str.push_str(&view[1..5]);
    //println!("literal bin str val: {:?}", lit_bin_str);
    lit_bin_str
}

fn step_operator_packet(ltid: &str, bit_str: &str) -> Vec<Packet> {
    let (p_start, mut p_end) = (7, 7);
    let sub_packets = match ltid {
        "0" => {
            p_end += 15;
            let tmp = &bit_str[p_start..p_end];
            let len_bit = usize::from_str_radix(tmp ,2).unwrap();
            parse_packets(&bit_str[p_end..p_end+(len_bit as usize)], None)
        },
        "1" => {
            p_end += 11;
            let tmp = &bit_str[p_start..p_end];
            let len_pkt = usize::from_str_radix(tmp ,2).unwrap();
            //println!("npackets = {:?}", len_pkt);
            parse_packets(&bit_str[p_end..], Some(len_pkt))
        }
        _ => unimplemented!()
    };
    println!("{:#?}\n", sub_packets);
    sub_packets
}

fn hex_to_bin(hex: Vec<String>) -> String {
    let mut bin_str = String::new();
    for s in hex {
        let i = i32::from_str_radix(&s, 16).unwrap();
        let smbin_str = &format!("{:04b}", i);
        bin_str.push_str(smbin_str);
        //println!("{:?} -> {:?}", s, smbin_str);
    }
    bin_str
}
