//! Oasis RPC interface.
use ethereum_types::Address;
use jsonrpc_core::BoxFuture;
use jsonrpc_derive::rpc;

use bytes::Bytes;
use common_types::{BlockNumber};
use ethereum_types::{H160, H256, U64};

#[rpc(server)]
pub trait Oasis {
    type Metadata;
    /// Returns the public key of a contract, given its address.
    #[rpc(name = "oasis_getPublicKey")]
    fn public_key(&self, a: Address) -> BoxFuture<Option<RpcPublicKeyPayload>>;

    /// Gets the expiration timestamp for a contract.
    /// The value is a Unix timestamp (seconds since the epoch).
    #[rpc(name = "oasis_getExpiry")]
    fn get_expiry(&self, h: H160, b: Option<BlockNumber>) -> BoxFuture<u64>;

    /// Sends a signed transaction, and returns the transaction hash,
    /// status code and return value.
    #[rpc(name = "oasis_invoke")]
    fn invoke(&self, b: Bytes) -> BoxFuture<RpcExecutionPayload>;
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RpcExecutionPayload {
    /// Transaction hash.
    #[serde(rename = "transactionHash")]
    pub transaction_hash: H256,
    /// Status code.
    #[serde(rename = "status")]
    pub status_code: U64,
    /// Return value.
    pub output: Bytes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcPublicKeyPayload {
    /// Public key of the contract.
    pub public_key: Bytes,
    /// Checksum of the key manager state.
    pub checksum: Bytes,
    /// Signature from the key manager authenticating the public key,
    /// i.e., Sign(ssk, (pk, t).
    pub signature: Bytes,
}