use roxmltree::Node;

use crate::xsd_model::elements::ElementType;
use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::Annotation;
use crate::xsd_model::LocalSimpleType;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::groups::simple_restriction_model::SimpleRestrictionModel;
use crate::xsd_model::complex_types::facet::Facet;

impl<'a> Facet<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for ch in node.children().filter(|n| n.is_element()){
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                _ => return Err(format!("Invalid child node for xsd:facet type: {:?}", node))
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "value" => res.value = attr.value(),
                "fixed" => res.fixed = attr.value().parse().map_err(|er| er.to_string())?,
                _ => res.attributes.push(attr.clone())
            };
        }

        Ok(res)
    }
}

