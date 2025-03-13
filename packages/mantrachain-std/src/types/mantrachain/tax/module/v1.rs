use osmosis_std_derive::CosmwasmExt;
/// Module is the config object for the module.
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
#[proto_message(type_url = "/mantrachain.tax.module.v1.Module")]
pub struct Module {
    #[prost(string, tag = "1")]
    pub fee_collector_name: ::prost::alloc::string::String,
}
