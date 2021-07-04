struct enum State{
    Closed,
    Listen,
    //SynRcvd,
    //Estab,
}
impl Default for State{
    fn default ()->self{
        State::Closed
    }
}

impl State{
    pub fn on_packet<'a>(
        &mut self,
        nic:&mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ){
        let mut buf = [0u8,1500];
        let mut written = 0;
        let mut unwritten = &mut buf[...];
        match *self{
            State::Closed =>{
                return;
            }
            State::Listem =>{
                if !tcph.syn(){
                    // only expected SYN packet
                    return;
                }
            // need to establish a SYN packet
            let syn_ack =etherparse::TcpHeader::new(
                tcph.destination_port,
                tcph.source_port,
                unimplemented!(),
                unimplemented!(),
            );
            syn_ack.syn = true;
            syn_ack.ack = true;
            let mut ipv4 = etherparse::Ipv4Header::new(
                syn_ack.header_len(),
                64,
                etherparse::IpTrafficClass::Tcp,
                [
                    iph.destination()[0],
                    iph.destination()[1],
                    iph.destination()[2],
                    iph.destination()[3],
                ],
                [
                    iph.source()[0],
                    iph.source()[1],
                    iph.source()[2],
                    iph.source()[3],
                ],
            );
            }
        }
        // write the headers
        let unwritten = {
            let mut unwritten = & mut buf [..];
            ip.write (&mut unwritten);
            syn_ack.write(&mut unwritten);
            unwritten.len()
        };
        nic.send(&buf[..unwritten]);
    }

