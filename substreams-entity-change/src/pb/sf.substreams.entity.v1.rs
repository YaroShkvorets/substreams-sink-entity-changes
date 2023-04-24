// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityChanges {
    #[prost(message, repeated, tag="5")]
    pub entity_changes: ::prost::alloc::vec::Vec<EntityChange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityChange {
    #[prost(string, tag="1")]
    pub entity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// Deprecated, this is not used within `graph-node`.
    #[prost(uint64, tag="3")]
    pub ordinal: u64,
    #[prost(enumeration="entity_change::Operation", tag="4")]
    pub operation: i32,
    #[prost(message, repeated, tag="5")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
}
/// Nested message and enum types in `EntityChange`.
pub mod entity_change {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        /// Protobuf default should not be used, this is used so that the consume can ensure that the value was actually specified
        Unset = 0,
        Create = 1,
        Update = 2,
        Delete = 3,
        Final = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityChangeAtBlockNum {
    #[prost(message, optional, tag="1")]
    pub entity_change: ::core::option::Option<EntityChange>,
    #[prost(uint64, tag="2")]
    pub block_num: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof="value::Typed", tags="1, 2, 3, 4, 5, 6, 10")]
    pub typed: ::core::option::Option<value::Typed>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Typed {
        #[prost(int32, tag="1")]
        Int32(i32),
        #[prost(string, tag="2")]
        Bigdecimal(::prost::alloc::string::String),
        #[prost(string, tag="3")]
        Bigint(::prost::alloc::string::String),
        #[prost(string, tag="4")]
        String(::prost::alloc::string::String),
        #[prost(string, tag="5")]
        Bytes(::prost::alloc::string::String),
        #[prost(bool, tag="6")]
        Bool(bool),
        //reserved 7 to 9;  // For future types

        #[prost(message, tag="10")]
        Array(super::Array),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Array {
    #[prost(message, repeated, tag="1")]
    pub value: ::prost::alloc::vec::Vec<Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub new_value: ::core::option::Option<Value>,
    /// Deprecated, this is not used within `graph-node`.
    #[prost(message, optional, tag="5")]
    pub old_value: ::core::option::Option<Value>,
}
/// Encoded file descriptor set for the `sf.substreams.entity.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb1, 0x17, 0x0a, 0x16, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2f, 0x76, 0x31, 0x2f, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x73, 0x66, 0x2e,
    0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x2e, 0x76, 0x31, 0x22, 0x5d, 0x0a, 0x0d, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x73, 0x12, 0x4c, 0x0a, 0x0e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f,
    0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e,
    0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x65, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x52, 0x0d, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e,
    0x67, 0x65, 0x73, 0x22, 0x9e, 0x02, 0x0a, 0x0c, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x0e, 0x0a, 0x02,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x18, 0x0a, 0x07,
    0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6f,
    0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x12, 0x4d, 0x0a, 0x09, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2f, 0x2e, 0x73, 0x66, 0x2e, 0x73,
    0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x2e, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x09, 0x6f, 0x70, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x36, 0x0a, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18,
    0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x2e,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x52, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x22, 0x45, 0x0a,
    0x09, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e,
    0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x10,
    0x01, 0x12, 0x0a, 0x0a, 0x06, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x10, 0x02, 0x12, 0x0a, 0x0a,
    0x06, 0x44, 0x45, 0x4c, 0x45, 0x54, 0x45, 0x10, 0x03, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x49, 0x4e,
    0x41, 0x4c, 0x10, 0x04, 0x22, 0x81, 0x01, 0x0a, 0x16, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x41, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4e, 0x75, 0x6d, 0x12,
    0x4a, 0x0a, 0x0d, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31,
    0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x0c, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x4e, 0x75, 0x6d, 0x22, 0xe4, 0x01, 0x0a, 0x05, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x12, 0x16, 0x0a, 0x05, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x05, 0x48, 0x00, 0x52, 0x05, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x12, 0x20, 0x0a, 0x0a, 0x62, 0x69,
    0x67, 0x64, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00,
    0x52, 0x0a, 0x62, 0x69, 0x67, 0x64, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x12, 0x18, 0x0a, 0x06,
    0x62, 0x69, 0x67, 0x69, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x06,
    0x62, 0x69, 0x67, 0x69, 0x6e, 0x74, 0x12, 0x18, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x12, 0x16, 0x0a, 0x05, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x48,
    0x00, 0x52, 0x05, 0x62, 0x79, 0x74, 0x65, 0x73, 0x12, 0x14, 0x0a, 0x04, 0x62, 0x6f, 0x6f, 0x6c,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x04, 0x62, 0x6f, 0x6f, 0x6c, 0x12, 0x36,
    0x0a, 0x05, 0x61, 0x72, 0x72, 0x61, 0x79, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e,
    0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x65, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x72, 0x72, 0x61, 0x79, 0x48, 0x00, 0x52,
    0x05, 0x61, 0x72, 0x72, 0x61, 0x79, 0x42, 0x07, 0x0a, 0x05, 0x74, 0x79, 0x70, 0x65, 0x64, 0x22,
    0x3d, 0x0a, 0x05, 0x41, 0x72, 0x72, 0x61, 0x79, 0x12, 0x34, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x76,
    0x31, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xbb,
    0x01, 0x0a, 0x05, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x40, 0x0a, 0x09,
    0x6e, 0x65, 0x77, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1e, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x48,
    0x00, 0x52, 0x08, 0x6e, 0x65, 0x77, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x88, 0x01, 0x01, 0x12, 0x40,
    0x0a, 0x09, 0x6f, 0x6c, 0x64, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1e, 0x2e, 0x73, 0x66, 0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x73, 0x2e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x48, 0x01, 0x52, 0x08, 0x6f, 0x6c, 0x64, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x88, 0x01, 0x01,
    0x42, 0x0c, 0x0a, 0x0a, 0x5f, 0x6e, 0x65, 0x77, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x0c,
    0x0a, 0x0a, 0x5f, 0x6f, 0x6c, 0x64, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x3f, 0x5a, 0x3d,
    0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x69, 0x6e, 0x67, 0x66, 0x61, 0x73, 0x74, 0x2f, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x73, 0x2d, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2d, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x2f, 0x70, 0x62, 0x3b, 0x70, 0x62, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4a, 0xcc, 0x0e,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x36, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x03, 0x00, 0x54, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x03, 0x00,
    0x54, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x05, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x06, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x06,
    0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x18, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x29, 0x2a, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x09, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x09, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x0a, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x09, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x12, 0x13, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0b, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0b, 0x0e, 0x0f, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02,
    0x15, 0x1a, 0x33, 0x20, 0x44, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x2c, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x69, 0x6e, 0x20, 0x60, 0x67, 0x72, 0x61, 0x70, 0x68, 0x2d, 0x6e,
    0x6f, 0x64, 0x65, 0x60, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d,
    0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x13, 0x14,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12, 0x04, 0x0e, 0x02, 0x14, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x07, 0x10, 0x0a, 0x87, 0x01, 0x0a,
    0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x04, 0x0e, 0x22, 0x78, 0x20, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20,
    0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x20, 0x73, 0x6f, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e,
    0x73, 0x75, 0x6d, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x73, 0x75, 0x72, 0x65, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x77,
    0x61, 0x73, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x66, 0x69, 0x65, 0x64, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0f, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x0f, 0x0c, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x10, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x10, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x10, 0x0d, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x11, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x11, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x11, 0x0d, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x12, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x12, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x12, 0x0d, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x13,
    0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x13,
    0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x13,
    0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x15, 0x02, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x15, 0x02, 0x0b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x15, 0x0c, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x15, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x04, 0x12, 0x03, 0x16, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x16,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x16, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x16, 0x1a, 0x1b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x19, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x19, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x1a, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1a, 0x04,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x11, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x21, 0x22, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x1b, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x1b, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x1b, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x1e, 0x00, 0x2b, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x03, 0x08, 0x00, 0x12, 0x04, 0x1f, 0x02, 0x2a, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x08, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x20, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x20, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x0a,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x12, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x21, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x21, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x21, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03,
    0x22, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x22, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x22, 0x14, 0x15, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x23, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x23, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x23, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x23, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x24, 0x04,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x24, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x24, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x24, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x05, 0x12, 0x03, 0x25, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x25, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x25, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x25,
    0x10, 0x11, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12, 0x03, 0x29, 0x04, 0x15, 0x32,
    0x26, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x20, 0x37, 0x20, 0x74, 0x6f, 0x20, 0x39,
    0x3b, 0x20, 0x20, 0x2f, 0x2f, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x66, 0x75, 0x74, 0x75, 0x72, 0x65,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x06,
    0x12, 0x03, 0x29, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x29, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x29, 0x12,
    0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2d, 0x00, 0x2f, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x2e, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x2e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2e,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x11, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x19, 0x1a, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x31, 0x00, 0x36, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05,
    0x01, 0x12, 0x03, 0x31, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03,
    0x32, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x32, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x09, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x10, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x33, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x33, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x33, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x33,
    0x1d, 0x1e, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x35, 0x02, 0x1f, 0x1a,
    0x33, 0x20, 0x44, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x69, 0x6e, 0x20, 0x60, 0x67, 0x72, 0x61, 0x70, 0x68, 0x2d, 0x6e, 0x6f, 0x64,
    0x65, 0x60, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x35,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x35, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x35, 0x11, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x35, 0x1d, 0x1e, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
