pub mod v1beta1;
use osmosis_std_derive::CosmwasmExt;
/// Params defines the parameters for the tokenfactory module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.Params")]
pub struct Params {
    /// DenomCreationFee defines the fee to be charged on the creation of a new
    /// denom. The fee is drawn from the MsgCreateDenom's sender account, and
    /// transferred to the community pool.
    #[prost(message, repeated, tag = "1")]
    pub denom_creation_fee: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// DenomCreationGasConsume defines the gas cost for creating a new denom.
    /// This is intended as a spam deterrence mechanism.
    ///
    /// See: <https://github.com/CosmWasm/token-factory/issues/11>
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub denom_creation_gas_consume: u64,
    /// FeeCollectorAddress is the address where fees collected from denom creation
    /// are sent to
    #[prost(string, tag = "3")]
    pub fee_collector_address: ::prost::alloc::string::String,
}
