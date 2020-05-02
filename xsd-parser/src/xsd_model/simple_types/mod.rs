pub mod any_uri;
pub mod base64binary;
pub mod block_set;
pub mod date;
pub mod datetime;
pub mod decimal;
pub mod derivation_set;
pub mod duration;
pub mod form_choice;
pub mod full_derivation_set;
pub mod gday;
pub mod gmonth;
pub mod gmonthday;
pub mod gyear;
pub mod gyearmonth;
pub mod id;
pub mod integer;
pub mod language;
pub mod ncname;
pub mod negative_integer;
pub mod non_negative_integer;
pub mod non_positive_integer;
pub mod positive_integer;
pub mod public;
pub mod qname;
pub mod simple_derivation_set;
pub mod time;
pub mod token;
pub mod boolean;

pub type AnySimpleType<'a> = &'a str;
pub type Id<'a> = Option<id::Id<'a>>;
