use std::borrow::Cow;

use crate::parser::{types::{Enum, EnumCase, Facet}, xsd_elements::FacetType};
use crate::generator::Generator;
use inflector::cases::{pascalcase::to_pascal_case, snakecase::to_snake_case};

pub trait Validate {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

pub trait ValidateGenerator {}

pub fn gen_validate_impl(name: &str, body: &str) -> String {
    if body.is_empty() {
        format!(r#"impl Validate for {name} {{}}"#, name = name)
    } else {
        format!(
            r#"impl Validate for {name} {{
    fn validate(&self) -> Result<(), String> {{ {body}
        Ok(())
    }}
}}
"#,
            name = name,
            body = body
        )
    }
}

pub fn gen_facet_validation(facet: &FacetType, name: &str, typename: &str) -> Cow<'static, str> {
    match facet {
        FacetType::Enumeration(_) => "".into(),
        FacetType::FractionDigits(_) => "".into(),
        FacetType::Length(value) => gen_length_validation(value.as_str(), name).into(),
        FacetType::MaxExclusive(value) => {
            gen_max_exclusive_validation(value.as_str(), name, typename).into()
        }
        FacetType::MaxInclusive(value) => {
            gen_max_inclusive_validation(value.as_str(), name, typename).into()
        }
        FacetType::MaxLength(value) => gen_max_length_validation(value.as_str(), name).into(),
        FacetType::MinExclusive(value) => {
            gen_min_exclusive_validation(value.as_str(), name, typename).into()
        }
        FacetType::MinInclusive(value) => {
            gen_min_inclusive_validation(value.as_str(), name, typename).into()
        }
        FacetType::MinLength(value) => gen_min_length_validation(value.as_str(), name).into(),
        _ => "".into(), // TODO: All Facet Types
    }
}

pub fn gen_validation_functions(facets: &Vec<&FacetType>, name: &str) -> String {
    let mut res = vec![];
    let mut frac_digit = None;
    let mut tot_digit = None;
    for facet in facets {
        match facet {
            FacetType::FractionDigits(val) => { frac_digit = Some(val) },
            FacetType::TotalDigits(val) => { tot_digit = Some(val) },
            FacetType::Pattern(val) => { res.push(gen_regex_validation(val, name)) },
            _ => {}
        }
    }
    match (frac_digit, tot_digit) {
        (Some(f_val), Some(t_val)) => { res.push(gen_digits_validation(f_val, t_val, name)); },
        _ => {},
    }
    res.join("\n\n")
}

pub fn gen_validation_macro(facets: &Vec<&FacetType>, name: &str, indent: &str) -> String {
    let mut length_vec = vec![];
    let mut range_vec = vec![];
    let mut regex_vec = vec![];
    let mut custom_vec = vec![];
    for facet in facets {
        gen_length_validation_macro(&facet, &mut length_vec);
        gen_range_validation_macro(&facet, &mut range_vec);
        gen_custom_validation_macro(&facet, name, &mut custom_vec);
        gen_regex_validation_macro(&facet, name, &mut regex_vec);
    }
    let length_str = if length_vec.is_empty() { None } else { Some(format!("length({})", length_vec.join(", "))) };
    let range_str = if range_vec.is_empty() { None } else { Some(format!("range({})", range_vec.join(", "))) };
    let regex_str = if regex_vec.is_empty() { None } else { Some(format!("regex({})", regex_vec.join(", "))) };
    let custom_str = if custom_vec.is_empty() { None } else { Some(format!("custom({})", custom_vec.join(", "))) };
    let res: Vec<String> = vec![length_str, range_str, regex_str, custom_str].into_iter().filter_map(|s| s).collect();
    if res.is_empty() { String::new() } else {
        format!("{indent}#[validate({str})]\n", str = res.join(", "))
    }
}

pub fn gen_length_validation_macro(facet: &FacetType, list: &mut Vec<String>) {
    match facet {
        FacetType::Length(val) => { list.push(format!("equal = {val}")); },
        FacetType::MaxLength(val) => { list.push(format!("max = {val}")); },
        FacetType::MinLength(val) => { list.push(format!("min = {val}")); },
        _ => {},
    }
}

pub fn gen_range_validation_macro(facet: &FacetType, list: &mut Vec<String>) {
    match facet {
        FacetType::MaxExclusive(val) => { list.push(format!("exclusive_max = {val}")); },
        FacetType::MinExclusive(val) => { list.push(format!("exclusive_min = {val}")); },
        FacetType::MaxInclusive(val) => { list.push(format!("max = {val}")); },
        FacetType::MinInclusive(val) => { list.push(format!("min = {val}")); },
        _ => {}
    }
}

pub fn gen_custom_validation_macro(facet: &FacetType, name: &str, list: &mut Vec<String>) {
    let name = to_snake_case(name);
    match facet {
        FacetType::FractionDigits(_) => { list.push(format!("function = {name}_digits_validator")); },
        // FacetType::TotalDigits(_) => { list.push(format!("function = {name}_total_digits_validator")); },
        _ => {}
    }
}

pub fn gen_regex_validation_macro(facet: &FacetType, name: &str, list: &mut Vec<String>) {
    let name = to_snake_case(name);
    match facet {
        FacetType::Pattern(_) => { list.push(format!("path = *{regex_name}", regex_name = format!("{name}_regex").to_ascii_uppercase())); },
        _ => {}
    }
}

pub fn gen_digits_validation(frac_value: &str, total_value: &str, name: &str) -> String {
    let name = to_snake_case(name);
    format!(
        r#"
fn {name}_digits_validator(num: &BigDecimal) -> Result<(), ValidationError> {{
    if num.fractional_digit_count() > {frac_value} {{
        let mut err = ValidationError::new("fractional_digits");
        err.add_param("max".into(), &{frac_value});
        err.add_param("value".into(), &num);
        return Err(err);
    }}
    if num.digits() > {total_value} {{
        let mut err = ValidationError::new("total_digits");
        err.add_param("max".into(), &{total_value});
        err.add_param("value".into(), &num);
        return Err(err);
    }}
    Ok(())
}}
        "#,
    )
}

pub fn gen_regex_validation(value: &str, name: &str) -> String {
    let name = to_snake_case(name);
    format!(
        r#"
static {regex_name}: Lazy<Regex> = Lazy::new(|| {{
    Regex::new(r"{value}").unwrap()
}});
        "#,
        regex_name = format!("{name}_regex").to_ascii_uppercase()
    )
}

pub fn gen_enum_validation(entity: &Enum, name: &str, gen: &Generator) -> String {
    let indent = gen.base().indent();
    let cases: String = entity.cases.iter().map(|c| gen_enum_case_validation(c)).filter_map(|s| s).collect::<Vec<String>>().join(&format!("\n{indent}{indent}{indent}"));
    let default_case = if cases.is_empty() { "_ => {}," } else { "" };
    format!(
        r#"
impl Validate for {name} {{
    fn validate(&self) -> Result<(), ValidationErrors> {{
        let mut err = ValidationErrors::new();
        match self {{
            {cases}
            Self::__Unknown__(val) => {{
                let mut field_err = ValidationError::new("unknown_enum");
                field_err.add_param("value".into(), &val);
                err.add("unknown", field_err);
            }},
            {default_case}
        }}
        if err.is_empty() {{ Ok(()) }} else {{ Err(err) }}
    }}
}}
        "#,
    )
}

fn gen_enum_case_validation(entity: &EnumCase) -> Option<String> {
    let name = to_pascal_case(&entity.name);
    match entity.type_name {
        Some(_) => Some(format!("Self::{name}(val) => {{ err.merge_self(\"{name}\", val.validate()); }},")),
        None => None
    }
}

fn gen_max_exclusive_validation(value: &str, name: &str, typename: &str) -> String {
    format!(
        r#"
        if self.{name} >= "{value}".parse::<{typename}>().unwrap() {{
            return Err(format!("MaxExclusive validation error: invalid value of {name}! \nExpected: {name} < {value}.\nActual: {name} == {{}}", self.{name}));
        }}"#,
        name = name,
        value = value,
        typename = typename
    )
}

fn gen_max_inclusive_validation(value: &str, name: &str, typename: &str) -> String {
    format!(
        r#"
        if self.{name} > "{value}".parse::<{typename}>().unwrap() {{
            return Err(format!("MaxInclusive validation error: invalid value of {name}! \nExpected: {name} <= {value}.\nActual: {name} == {{}}", self.{name}));
        }}"#,
        name = name,
        value = value,
        typename = typename
    )
}

fn gen_length_validation(value: &str, name: &str) -> String {
    let value: u32 = value.parse().unwrap();
    format!(
        r#"
        if self.{name}.len() != {value} {{
            return Err(format!("Length validation error. \nExpected: {name} length == {value} \nActual: {name} length == {{}}", self.{name}.len()));
        }}"#,
        name = name,
        value = value
    )
}

fn gen_max_length_validation(value: &str, name: &str) -> String {
    let value: u32 = value.parse().unwrap();
    format!(
        r#"
        if self.{name}.len() > {value} {{
            return Err(format!("MaxLength validation error. \nExpected: {name} length <= {value} \nActual: {name} length == {{}}", self.{name}.len()));
        }}"#,
        name = name,
        value = value
    )
}

fn gen_min_exclusive_validation(value: &str, name: &str, typename: &str) -> String {
    format!(
        r#"
        if self.{name} <= "{value}".parse::<{typename}>().unwrap() {{
            return Err(format!("MinExclusive validation error: invalid value of {name}! \nExpected: {name} > {value}.\nActual: {name} == {{}}", self.{name}));
        }}"#,
        name = name,
        value = value,
        typename = typename
    )
}

fn gen_min_inclusive_validation(value: &str, name: &str, typename: &str) -> String {
    format!(
        r#"
        if self.{name} < "{value}".parse::<{typename}>().unwrap() {{
            return Err(format!("MinInclusive validation error: invalid value of {name}! \nExpected: {name} >= {value}.\nActual: {name} == {{}}", self.{name}));
        }}"#,
        name = name,
        value = value,
        typename = typename
    )
}

fn gen_min_length_validation(value: &str, name: &str) -> String {
    let value: u32 = value.parse().unwrap();
    if value == 0 {
        return "".into();
    }

    format!(
        r#"
        #[allow(clippy::len_zero)]
        if self.{name}.len() < {value} {{
            return Err(format!("MinLength validation error. \nExpected: {name} length >= {value} \nActual: {name} length == {{}}", self.{name}.len()));
        }}"#,
        name = name,
        value = value
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_max_exclusive_validation() {
        let expected = r#"
        if self.count >= "5".parse::<i32>().unwrap() {
            return Err(format!("MaxExclusive validation error: invalid value of count! \nExpected: count < 5.\nActual: count == {}", self.count));
        }"#;
        assert_eq!(gen_max_exclusive_validation("5", "count", "i32"), expected);
    }

    #[test]
    fn test_gen_max_inclusive_validation() {
        let expected = r#"
        if self.count > "5".parse::<i32>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of count! \nExpected: count <= 5.\nActual: count == {}", self.count));
        }"#;
        assert_eq!(gen_max_inclusive_validation("5", "count", "i32"), expected);
    }

    #[test]
    fn test_gen_length_validation() {
        let expected = r#"
        if self.name.len() != 50 {
            return Err(format!("Length validation error. \nExpected: name length == 50 \nActual: name length == {}", self.name.len()));
        }"#;
        assert_eq!(gen_length_validation("50", "name"), expected);
    }

    #[test]
    fn test_gen_max_length_validation() {
        let expected = r#"
        if self.name.len() > 50 {
            return Err(format!("MaxLength validation error. \nExpected: name length <= 50 \nActual: name length == {}", self.name.len()));
        }"#;
        assert_eq!(gen_max_length_validation("50", "name",), expected);
    }

    #[test]
    fn test_gen_min_exclusive_validation() {
        let expected = r#"
        if self.count <= "5".parse::<i32>().unwrap() {
            return Err(format!("MinExclusive validation error: invalid value of count! \nExpected: count > 5.\nActual: count == {}", self.count));
        }"#;
        assert_eq!(gen_min_exclusive_validation("5", "count", "i32"), expected);
    }

    #[test]
    fn test_gen_min_inclusive_validation() {
        let expected = r#"
        if self.count < "5".parse::<i32>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of count! \nExpected: count >= 5.\nActual: count == {}", self.count));
        }"#;
        assert_eq!(gen_min_inclusive_validation("5", "count", "i32"), expected);
    }

    #[test]
    fn test_gen_min_length_validation() {
        let expected = r#"
        #[allow(clippy::len_zero)]
        if self.name.len() < 50 {
            return Err(format!("MinLength validation error. \nExpected: name length >= 50 \nActual: name length == {}", self.name.len()));
        }"#;
        assert_eq!(gen_min_length_validation("50", "name"), expected);
    }

    #[test]
    fn test_gen_min_length_zero_validation() {
        let expected = "";
        assert_eq!(gen_min_length_validation("0", "name"), expected);
    }
}
