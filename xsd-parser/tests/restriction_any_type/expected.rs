// pub type AppSequence = AppSequenceType;
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(prefix = "tns", namespace = "tns: http://schemas.xmlsoap.org/ws/2005/04/discovery")]
pub struct AppSequenceType {
    #[serde(attribute, rename = "InstanceId")]
    pub instance_id: u32,

    #[serde(attribute, rename = "SequenceId")]
    pub sequence_id: Option<String>,

    #[serde(attribute, rename = "MessageNumber")]
    pub message_number: u32,
}

impl Validate for AppSequenceType {}
