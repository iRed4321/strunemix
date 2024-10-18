//! [![Crates.io](https://img.shields.io/crates/v/strunemix.svg)](https://crates.io/crates/strunemix)
//! [![Docs](https://docs.rs/strunemix/badge.svg)](https://docs.rs/strunemix)
//! 
//! Strunemix allows to build a struct with a form of its fields, by deriving enums of them.
//! 
//! # Example
//! 
//! ```rust
//! use strunemix::*;
//! 
//! #[derive(Debug, PartialEq, Strunemix)]
//! #[strunemix_derive_data(Debug, PartialEq, Clone)]
//! struct Person {
//!    pseudo: String,
//!    age: i32,
//! }
//! 
//! let person = Person {
//!     pseudo: "BestPseudo".to_string(),
//!     age: 42
//! };
//! 
//! // Attributes names are turned to an enum
//! assert_eq!(Person::as_attr_name_array(), [PersonAttrName::Pseudo, PersonAttrName::Age]);
//! assert_eq!(PersonAttrName::Pseudo.get_str(), "pseudo");
//! assert_eq!(PersonAttrName::Age.get_str(), "age");
//! 
//! // Attributes data are turned to an enum
//! let pseudo_data = PersonAttrData::Pseudo("BestPseudo".to_string());
//! let age_data = PersonAttrData::Age(42);
//! assert_eq!(person.to_attr_data_array(), [pseudo_data, age_data]);
//! 
//! // Move between the struct and a form of it
//! let mut form = Person::empty_form::<()>();
//! form.set_data(PersonAttrName::Pseudo, PersonAttrData::Pseudo("BeckyTheBest".to_string()));
//! form.set_data(PersonAttrName::Age, PersonAttrData::Age(25));
//! 
//! let becky = Person::from_form(form).unwrap();
//! 
//! assert_eq!(becky, Person { pseudo: "BeckyTheBest".to_string(), age: 25 });
//! ```
//! If you want to build the attribute data from string values, you must implement the [`StrunemixParsableData`] trait to handle the conversion from the string data to the struct fields.
//! 
//! ```rust
//! # use strunemix::*;
//! # #[derive(Debug, PartialEq, Strunemix)]
//! # #[strunemix_derive_data(Debug, PartialEq)]
//! # struct Person {
//! #   pseudo: String,
//! #   age: i32,
//! # }
//! // Implement the trait for the enum names
//! impl StrunemixParsableData<'_, PersonAttrData> for PersonAttrName {
//!   fn add_data(&self, data: &str) -> Result<PersonAttrData, StrunemixParseError> {
//!     match self {
//!       PersonAttrName::Pseudo => Ok(PersonAttrData::Pseudo(data.to_string())),
//!       PersonAttrName::Age => Ok(PersonAttrData::Age(data.parse()?))
//!     }
//!   }
//! }
//! 
//! fn main() -> Result<(), StrunemixError> {
//! // Build the attribute data from string values
//! let pseudo_expected = PersonAttrData::Pseudo("MyCoolPseudo".to_string());
//! let pseudo = "pseudo".field_of::<Person>()?.add_data("MyCoolPseudo")?;
//! assert_eq!(&pseudo_expected, &pseudo);
//! 
//! let mut form = Person::empty_form::<()>();
//! 
//! // Add the data to the form the way you want
//! 
//! // With the generated enums
//! form.set_data(PersonAttrName::Age, PersonAttrData::Age(42))?;
//! 
//! // or with the name as a string
//! form.set_data("age", PersonAttrData::Age(42))?;
//! 
//! // or with a string for the data
//! form.set_data_str(PersonAttrName::Pseudo, "MyCoolPseudo")?;
//! 
//! // or with only strings
//! form.set_data_str("pseudo", "MyCoolPseudo")?;
//! # let person = Person { pseudo: "MyCoolPseudo".to_string(), age: 42 };
//! # assert_eq!(person, Person::from_form(form)?);
//! # Ok(())
//! # }
//! ```


/// Implements the [`StrunemixTrait`] on a struct automatically and generate the necessary enums and traits implementations.
/// 
/// # Examples
/// 
/// ## Skip a field
/// 
/// `#[strunemix(skip)` can be used to skip a field from the generated enums and traits.
/// 
/// ```rust
/// use strunemix::*;
/// 
/// #[derive(Strunemix)]
/// struct Person {
///   pseudo: String,
///   age: i32,
///   #[strunemix(skip)]
///   phone: Option<String>,
/// }
/// 
/// assert_eq!(Person::FIELDS_COUNT, 2);
/// ```
/// 
/// **Special Case :**\
/// If you want to build the struct from an array of attributes data, the skipped fields have to be initialized with a default value.
/// To do so, you have to implement the [`Default`] trait on the struct and add the `#[strunemix_default]` attribute on the struct.
/// 
/// ```rust
/// 
/// use strunemix::*;
/// 
/// #[derive(Debug, PartialEq, Default, Strunemix)]
/// #[strunemix_default]
/// struct Person {
///   pseudo: String,
///   age: i32,
///   #[strunemix(skip)]
///   phone: Option<String>,
/// }
/// 
/// let expected = Person {pseudo: "John".to_string(), age: 42, phone: None};
/// 
/// let data = [PersonAttrData::Pseudo("John".to_string()), PersonAttrData::Age(42)];
/// let person = Person::from_attr_data_array(data).unwrap();
/// 
/// assert_eq!(person, expected);
/// ```
/// 
/// ## Derive Macros on the enums names
/// 
/// `#[strunemix_derive_name]` can be used to derive some macros on the generated enums names.
/// If the attribute is not used, the macros `Debug`, `PartialEq`, `Eq`, `Clone` and `Copy` are derived by default.
/// 
/// ```rust
/// use strunemix::*;
/// 
/// #[derive(Strunemix)]
/// #[strunemix_derive_name(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// struct Person {
///   pseudo: String,
///   age: i32,
///   phone: Option<String>,
/// }
/// ```
/// 
/// ## Derive Macros on the enums data
/// 
/// `#[strunemix_derive_data]` can be used to derive some macros on the generated enums data.
/// If the attribute is not used, no macros are derived by default.
/// 
/// ```rust
/// use strunemix::*;
/// 
/// #[derive(Strunemix)]
/// #[strunemix_derive_data(Debug, PartialEq, Eq, Clone)]
/// struct Person {
///   pseudo: String,
///   age: i32,
///   phone: Option<String>
/// }
/// 
/// let age = PersonAttrData::Age(42);
/// let age_clone = age.clone();
/// ```
/// 
/// ## Derive Macros on the enums names and data
/// 
/// `#[strunemix_derive]` can be used to derive some macros on both the generated enums names and data.
/// If the attribute is not used, the macros derived are the same as the ones derived on the enums names and data.
pub use strunemix_macros::Strunemix;

mod data;
mod top;
mod form;
mod name;
mod error;
    
pub use crate::name::*;
pub use crate::data::*;
pub use crate::form::*;
pub use crate::top::*;
pub use crate::error::*;