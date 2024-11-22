#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[serde(prefix = "tns")]
    pub min: i32,

    #[serde(prefix = "tns")]
    pub max: i32,
}
