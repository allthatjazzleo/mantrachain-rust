use neutron_std_derive::CosmwasmExt;
/// Module is the config object of the builder module.
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
#[proto_message(type_url = "/connect.marketmap.module.v2.Module")]
pub struct Module {
    /// Authority defines the custom module authority. If not set, defaults to the
    /// governance module.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// HooksOrder specifies the order of marketmap hooks and should be a list
    /// of module names which provide a marketmap hooks instance. If no order is
    /// provided, then hooks will be applied in alphabetical order of module names.
    #[prost(string, repeated, tag = "2")]
    pub hooks_order: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
