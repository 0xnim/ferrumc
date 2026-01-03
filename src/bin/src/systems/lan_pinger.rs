use ferrumc_config::server_config::get_global_config;
use rand::prelude::IndexedRandom;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::UdpSocket;
use tracing::trace;

pub struct LanPinger {
    socket: UdpSocket,
    addr: SocketAddrV4,
}

impl LanPinger {
    pub async fn new() -> std::io::Result<Self> {
        const ADDR: Ipv4Addr = Ipv4Addr::new(224, 0, 2, 60); // mojang's UDP multicast address
        const PORT: u16 = 4445;

        Ok(Self {
            socket: UdpSocket::bind("0.0.0.0:0").await?,
            addr: SocketAddrV4::new(ADDR, PORT),
        })
    }

    pub fn announcement(&self) -> String {
        let cfg = get_global_config();
        let motd = cfg.motd.choose(&mut rand::rng()).unwrap();
        let port = cfg.port;

        format!("[MOTD]{motd}[/MOTD][AD]{port}[/AD]")
    }

    pub async fn send(&mut self) {
        let announcement = self.announcement();

        // LAN broadcast failures are expected when not on a local network
        // (e.g., "No route to host" on macOS), so log at trace level only
        if let Err(err) = self
            .socket
            .send_to(announcement.as_bytes(), self.addr)
            .await
        {
            trace!("LAN broadcast unavailable: {err}");
        }
    }
}
