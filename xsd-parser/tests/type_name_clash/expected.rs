#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct BarType {
    #[serde(attribute, rename = "a")]
    pub a: Option<String>,
}

impl Validate for BarType {}


#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[serde(prefix = "tns", rename = "Bar")]
    pub bar: foo_type::BarType,
}

impl Validate for FooType {}

pub mod foo_type {
    use super::*;

    #[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
    #[serde(prefix = "tns", namespace = "tns: http://example.com")]
    pub struct BarType {
        #[serde(attribute, rename = "b")]
        pub b: Option<String>,

        #[serde(attribute, rename = "a")]
        pub a: Option<String>,

    }

    impl Validate for BarType {}
}
