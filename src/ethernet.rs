use rand::RngCore;
use smoltcp::wire;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0], octet[1], octet[2],
            octet[3], octet[4], octet[5]
        )
    }
}

impl MacAddress  {
    pub fn new() -> Self {
        let mut octets: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b_0000_0010; // ローカルアドレスのビットを1に設定
        octets[0] &= 0b_1111_1110; // ユニキャストビットを0にする
        Self(octets)
    }
}

impl Default for MacAddress {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MacAddress> for wire::EthernetAddress {
    fn from(val: MacAddress) -> Self {
        wire::EthernetAddress(val.0)
    }
}

