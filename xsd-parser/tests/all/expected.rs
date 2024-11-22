#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[serde(prefix = "tns", rename = "Once")]
    pub once: i32,

    #[serde(prefix = "tns", rename = "Optional")]
    pub optional: Option<i32>,

    #[serde(prefix = "tns", rename = "OnceSpecify")]
    pub once_specify: i32,

    #[serde(prefix = "tns", rename = "TwiceOrMore")]
    pub twice_or_more: Vec<i32>,
}

impl Validate for FooType {}

// pub type Foo = FooType;
