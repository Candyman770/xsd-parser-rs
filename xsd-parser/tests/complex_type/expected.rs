#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[serde(prefix = "tns", rename = "Min")]
    pub min: i32,

    #[serde(prefix = "tns", rename = "Max")]
    pub max: i32,
}

impl Validate for FooType {}
