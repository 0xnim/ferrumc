use std::fs::File;
use std::hint::black_box;
use std::io::Cursor;
use std::sync::{Arc, OnceLock};
use async_trait::async_trait;
use fastanvil::Region;
use ferrumc_macros::AutoGenName;
use simdnbt::borrow::Nbt;
use simdnbt::Deserialize;
use tokio::sync::RwLock;
use tracing::debug;
use tracing::field::debug;
use crate::net::{Connection, ConnectionWrapper, GET_WORLD};
use crate::net::packets::outgoing::chunk_data_and_light::ChunkDataAndUpdateLight;
use crate::net::systems::System;
use crate::utils::components::keep_alive::KeepAlive;
use crate::utils::components::player::Player;
use crate::utils::encoding::position::Position;
use crate::utils::prelude::*;
use crate::world::sweattypalms_impl::types::Chunk;

#[derive(AutoGenName)]
pub struct ChunkSender;

#[async_trait]
impl System for ChunkSender {
    async fn run(&self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let mut world = GET_WORLD().read().await;
            let query = world.query::<(Player, Position, ConnectionWrapper)>()
                .iter().collect::<Vec<_>>();

            for (_, (player, pos, conn)) in query {
                debug!("Sending chunk to player: {} with position: {}", player.get_username(), pos);
                let player_x = pos.x;
                let player_z = pos.z;
                if let Err(e) = send_chunks_around_player(conn.0.clone(), player_x, player_z).await {
                    debug!("Failed to send chunks to player: {}", e);
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}

async fn send_chunks_around_player(conn: Arc<RwLock<Connection>>, player_x: i32, player_z: i32) -> Result<()> {
    let chunk_x = player_x >> 4;
    let chunk_z = player_z >> 4;
    let render_distance = 1;

    let mut conn_write = conn.write().await;

    // for dx in -render_distance..=render_distance {
    //     for dz in -render_distance..=render_distance {
            debug!("Sending chunk at x: {} z: {}", chunk_x/* + dx*/, chunk_z /*+ dz*/);
            let x = chunk_x/* + dx*/;
            let z = chunk_z /*+ dz*/;
            let chunk = get_chunk(x, z).await?; // You need to implement this function
            let packet = ChunkDataAndUpdateLight::new(&chunk).await?;
            conn_write.send_packet(packet).await?;
        // }
    // }

    drop(conn_write);

    Ok(())
}

/*fn read_one() {
    let start = std::time::Instant::now();
    let mut region = get_region("r.-1.-2.mca");
    println!("Time taken to open region file: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let chunk_data = region.read_chunk(15, 30).expect("Failed to read chunk data").unwrap();

    write_chunk_to_file(&chunk_data);

    let nbt = read(&mut Cursor::new(&chunk_data)).expect("Failed to parse chunk data^1").unwrap();

    let data = Chunk::from_nbt(&nbt).expect("Failed to parse chunk data^2");
    black_box(data);
    println!("Time taken to read and parse one chunk: {:?}", start.elapsed());
}*/


#[allow(non_snake_case)]
fn GET_REGION() -> &'static RwLock<Region<File>> {
    static STATIC_REGION: OnceLock<RwLock<Region<File>>> = OnceLock::new();
    // TODO: Change this entirely!
    STATIC_REGION.get_or_init(|| RwLock::new(get_region("r.-1.-2.mca")))
}

fn get_region(file: &'static str) -> Region<File> {
    let region_file = File::open(file).expect("Failed to open region file");
    let reader = Region::from_stream(region_file).expect("Failed to create region reader");
    reader
}

async fn get_chunk(x: i32, z: i32) -> Result<Chunk> {
    // For now just read from that specific region
    let mut region = GET_REGION().write().await;
    // let chunk_data = region.read_chunk(x as usize, z as usize)?.ok_or(Error::ChunkNotFound(x, z))?;
    let chunk_data = region.read_chunk(15,30)?.ok_or(Error::ChunkNotFound(x, z))?;

    let nbt = simdnbt::borrow::read(&mut Cursor::new(&chunk_data))?;

    let nbt = match nbt {
        Nbt::Some(nbt) => { nbt }
        Nbt::None => {
            return Err(Error::InvalidNbt("Chunk data is empty".to_string()));
        }
    };

    let data = Chunk::from_nbt(&nbt)?;

    Ok(data)
}