use std::borrow::Cow;
use roxmltree::Namespace;

use crate::{
    generator::{validator::{gen_validate_impl, gen_enum_validation}, Generator},
    parser::types::{Enum, EnumSource},
};

pub trait EnumGenerator {
    fn generate(&self, entity: &Enum, gen: &Generator) -> String {
        let wrapper_name = self.get_name(entity, gen);
        let mut name = wrapper_name.clone();
        if entity.source == EnumSource::Restriction {
            name.push_str("Choice");
        }
        let default_case = format!(
            "impl Default for {name} {{\n\
            {indent}fn default() -> {name} {{\n\
            {indent}{indent}Self::__Unknown__(\"No valid variants\".into())\n\
            {indent}}}\n\
            }}",
            name = name,
            indent = gen.base().indent()
        );
        let wrapper_struct = format!(
            "{macros}\
            pub struct {wrapper_name} {{\n\
                {indent}#[serde(rename = \"$text\")]\n\
                {indent}#[validate(nested)]\n\
                {indent}pub {field_name}: {name}\n\
            }}\n\n",
            wrapper_name = wrapper_name,
            field_name = gen.base().format_name(&name),
            name = name,
            indent = gen.base().indent(),
            macros = self.macros(entity, gen, true),
        );
        format!(
            "{comment}{macros}\
            pub enum {name} {{\n\
                {cases}\n\
                {indent}__Unknown__({typename}),\n\
            }}\n\n\
            {validation}\n\n\
            {default}\n\n\
            {subtypes}\n\n\
            {wrapper_struct}",
            indent = gen.base().indent(),
            comment = self.format_comment(entity, gen),
            macros = self.macros(entity, gen, false),
            name = name,
            cases = self.cases(entity, gen),
            typename = self.get_type_name(entity, gen),
            default = default_case,
            subtypes = self.subtypes(entity, gen),
            // validation = self.validation(entity, gen),
            wrapper_struct = if entity.source == EnumSource::Restriction { &wrapper_struct } else { "" },
            validation = gen_enum_validation(&entity, &name, &gen),
        )
    }

    fn cases(&self, entity: &Enum, gen: &Generator) -> String {
        entity
            .cases
            .iter()
            .map(|case| gen.enum_case_gen().generate(case, gen))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn subtypes(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().join_subtypes(entity.subtypes.as_ref(), gen)
    }

    fn get_type_name(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().format_type_name(entity.type_name.as_str(), gen).into()
    }

    fn get_name(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().format_type_name(entity.name.as_str(), gen).into()
    }

    fn macros(&self, entity: &Enum, gen: &Generator, with_default: bool) -> Cow<'static, str> {
        if entity.source == EnumSource::Union {
            return "#[derive(PartialEq, Debug, UtilsUnionSerDe)]".into();
        }

        let derives = if with_default { "#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, Validate)]" } else { "#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]" };
        let _tns = gen.target_ns.borrow();
        let tns_ref: Option<Namespace> = None;
        match tns_ref {
            Some(tn) => match tn.name() {
                Some(name) => format!(
                    "{derives}#[serde(prefix = \"{prefix}\", namespace = \"{prefix}: {uri}\")]\n",
                    derives = derives,
                    prefix = name,
                    uri = tn.uri()
                ),
                None => format!(
                    "{derives}#[serde(namespace = \"{uri}\")]\n",
                    derives = derives,
                    uri = tn.uri()
                ),
            },
            None => format!("{derives}\n", derives = derives),
        }
        .into()
    }

    fn format_comment(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().format_comment(entity.comment.as_deref(), 0)
    }

    fn validation(&self, entity: &Enum, gen: &Generator) -> Cow<'static, str> {
        // Empty validation
        Cow::Owned(gen_validate_impl(self.get_name(entity, gen).as_str(), ""))
    }
}

pub struct DefaultEnumGen;
impl EnumGenerator for DefaultEnumGen {}
