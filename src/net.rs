use std::io;
use std::ops::Index;
use std::str::FromStr;

use tokio::net::UdpSocket;

pub struct MacAddress([u8; 6]);

impl FromStr for MacAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0u8; 6];
        let mut n = 0;

        for (i, v) in s.split(':').enumerate() {
            bytes[i] = u8::from_str_radix(v, 16).map_err(|e| e.to_string())?;
            n += 1;
        }

        if n == 6 {
            Ok(MacAddress(bytes))
        } else {
            Err("Invalid number of octets".to_owned())
        }
    }
}

impl Index<usize> for MacAddress {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

pub struct MagicPacket([u8; 102]);

impl MagicPacket {
    pub fn for_mac_address(mac_address: &MacAddress) -> MagicPacket {
        let mut packet = [0u8; 102];

        // 6 bytes of 0xff...
        for byte in packet.iter_mut().take(6) {
            *byte = 0xff;
        }

        // ...followed by 16 repetitions of the target mac address
        for i in 0..16 {
            for j in 0..6 {
                packet[6 + i * 6 + j] = mac_address[j];
            }
        }

        MagicPacket(packet)
    }

    pub fn bytes(&self) -> &[u8; 102] {
        &self.0
    }

    pub async fn broadcast(&self) -> Result<(), io::Error> {
        let mut socket = UdpSocket::bind("0.0.0.0:0").await?;

        socket.set_broadcast(true).unwrap();
        socket
            // TODO: Don't hardcode this address
            .send_to(self.bytes(), "192.168.1.255:9")
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_mac_address_parses_successfully() {
        let address: MacAddress = "01:23:45:ab:cd:ef".parse().unwrap();
        assert_eq!(address.0, [0x01, 0x23, 0x45, 0xab, 0xcd, 0xef]);
    }

    #[test]
    #[should_panic]
    fn mac_address_with_too_few_octets_does_not_parse() {
        let _: MacAddress = "01:23:45:ab".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn mac_address_with_too_many_octets_does_not_parse() {
        let _: MacAddress = "01:23:45:ab:cd:ef:45".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn mac_address_with_invalid_hex_does_not_parse() {
        let _: MacAddress = "01:23:45:ab:cd:eg".parse().unwrap();
    }
}
