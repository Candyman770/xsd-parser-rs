#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct BarType {
    #[serde(prefix = "tns", rename = "b")]
    pub b: i32,

    #[serde(prefix = "tns", rename = "c")]
    pub c: String,
}

impl Validate for BarType {}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[serde(prefix = "tns", rename = "a")]
    pub a: f64,

    #[serde(prefix = "tns", rename = "b")]
    pub b: i32,

    #[serde(prefix = "tns", rename = "c")]
    pub c: String,
}

impl Validate for FooType {}
