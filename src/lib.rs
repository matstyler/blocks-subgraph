mod pb;
mod utils;

use pb::blocks::{Block};

use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth;
use crate::utils::{format_hex};

#[substreams::handlers::map]
fn map_block(block: eth::v2::Block) -> Result<Block, substreams::errors::Error> {
    let header = block.header.as_ref().unwrap();

    Ok(Block {
        number: block.number,
        timestamp: header
            .timestamp
            .as_ref()
            .unwrap()
            .seconds as u64,
        size: block.size,
        transactions: block.transactions().count() as u64,
        hash: format_hex(&block.hash),
        gas_used: header.gas_used,
    })
}

#[substreams::handlers::map]
pub fn graph_out(block: Block) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    tables
        .create_row("Block", block.number.to_string())
        .set("number", block.number)
        .set("timestamp", block.timestamp)
        .set("size", block.size)
        .set("transactions", block.transactions)
        .set("hash", block.hash)
        .set("gas_used", block.gas_used);

    Ok(tables.to_entity_changes())
}
