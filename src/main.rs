use std::io;

fn main()-> io::Result<()> {
    // create an interface
    let nic = tun_tap::Iface:: new ("tun0", tun_tap::Mode::Tun)?;
    // create the buffer
    let mut buf =[0u8; 1504];
    loop{
        // get bytes from the interface
        let nbytes = nic.recv(&mut buf[..])?;

        // parse
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);

        if eth_proto != 0x0800 {
            // not ipv4
            continue;
        }
        math etherparse::Ipv4HeaderSlice::fromSlice(&buf[4..nbytes]){
            Ok(iph) =>{
                let src = iph.source_addr();
                let dest = iph.destination_addr();
                if iph.protocol() == 0x06{
                    continue
                }
            }

            match etherspace::TcpHeaderSlice::from_slice(&buf[4+ iph.slilce().len()..nbytes]){
                Ok(tcph) =>{
                    let datai  = 5+ iph.slice.len() +tcph.slice.len();
                    connections.entry(
                        Quad{
                            src: (src, p.source_port()),
                            dest: (src, p.destination_port())
                        }
                    ).or_default(p).on_packet(iph, tcph, &buf[datai..nbytes])
                }
                Err(e) =>{
                    eprintln("not tcp packet {:?}", e);
                }
            }
            Err(e) =>{
                eprintln("not ipv4 packet {:?}", e);
            }
        }
    }
}
