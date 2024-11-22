#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct BarType {
    #[serde(prefix = "tns", rename = "aa")]
    pub aa: i32,

    #[serde(prefix = "tns", rename = "bb")]
    pub bb: String,
}

impl Validate for BarType {}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[serde(prefix = "tns", rename = "Messages")]
    pub messages: foo_type::MessagesType,
}

impl Validate for FooType {}

pub mod foo_type {
    use super::*;

    #[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
    #[serde(prefix = "tns", namespace = "tns: http://example.com")]
    pub struct MessagesType {
        #[serde(prefix = "tns", rename = "a")]
        pub a: String,

        #[serde(prefix = "tns", rename = "aa")]
        pub aa: i32,

        #[serde(prefix = "tns", rename = "bb")]
        pub bb: String,
    }

    impl Validate for MessagesType {}
}
