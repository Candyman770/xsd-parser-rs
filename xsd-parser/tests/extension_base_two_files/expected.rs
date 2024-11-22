#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(
    prefix = "tns",
    namespace = "tns: http://example.com",
    namespace = "tns2: http://other.example.com"
)]
pub struct FooType {
    #[serde(prefix = "tns", rename = "a")]
    pub a: f64,

    #[serde(prefix = "tns2", rename = "b")]
    pub b: i32,

    #[serde(prefix = "tns2", rename = "c")]
    pub c: String,
}
