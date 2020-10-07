use bigdecimal::BigDecimal;

use crate::schema;
use schema::chunks;

#[derive(Insertable, Clone, Debug)]
pub struct Chunk {
    pub block_hash: String,
    pub hash: String,
    pub shard_id: BigDecimal,
    pub signature: String,
    pub gas_limit: BigDecimal,
    pub gas_used: BigDecimal,
}

impl Chunk {
    pub fn from_chunk_view(
        chunk_view: &near_indexer::IndexerChunkView,
        block_hash: &near_indexer::near_primitives::hash::CryptoHash,
    ) -> Self {
        Self {
            block_hash: block_hash.to_string(),
            hash: chunk_view.header.chunk_hash.to_string(),
            shard_id: chunk_view.header.shard_id.into(),
            signature: chunk_view.header.signature.to_string(),
            gas_limit: chunk_view.header.gas_limit.into(),
            gas_used: chunk_view.header.gas_used.into(),
        }
    }
}