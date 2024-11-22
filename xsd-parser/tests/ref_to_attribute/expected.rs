#[derive(Default, Clone, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Id(pub String);

impl Validate for Id {}
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[serde(attribute, prefix = "tns", rename = "id")]
    pub id: Option<Id>,
}

impl Validate for FooType {}
