//! Strunemix allows to work with structs as if they were enums.
//! 
//! # Example
//! 
//! ```rust
//! use strunemix::*;
//! 
//! #[derive(Debug, PartialEq, Default, Clone, Strunemix)]
//! #[strunemix_derive_data(Debug, PartialEq, Clone)]
//! struct Person {
//!    pseudo: String,
//!    phone: Option<String>,
//!    age: i32,
//! }
//! 
//! let person = Person {pseudo: "MyCoolPseudo".to_string(), phone: Some("123456789".to_string()), age: 42};
//! 
//! let pseudo_name = PersonAttrName::Pseudo;
//! let phone_name = PersonAttrName::Phone;
//! let age_name = PersonAttrName::Age;
//! 
//! assert_eq!(Person::as_attr_name_array(), [pseudo_name, phone_name, age_name]);
//! assert_eq!(pseudo_name.name(), "pseudo");
//! assert_eq!(phone_name.name(), "phone");
//! assert_eq!(age_name.name(), "age");
//! 
//! let pseudo_data = PersonAttrData::Pseudo("MyCoolPseudo".to_string());
//! let phone_data = PersonAttrData::Phone(Some("123456789".to_string()));
//! let age_data = PersonAttrData::Age(42);
//! 
//! assert_eq!(person.clone().to_attr_data_array(), [pseudo_data.clone(), phone_data.clone(), age_data.clone()]);
//! 
//! let personcopy = Person::from_attr_data_array([pseudo_data, phone_data, age_data]).unwrap();
//! 
//! assert_eq!(person, personcopy);
//! ```
//! If you want to build the attribute data from string values, you must implement the [`StrunemixParsableData`] trait to handle the conversion from the data to the struct fields.
//! 
//! ```rust
//! # use strunemix::*;
//! # #[derive(Debug, PartialEq, Default, Strunemix)]
//! # #[strunemix_derive_data(Debug, PartialEq)]
//! # struct Person {
//! #   pseudo: String,
//! #   phone: Option<String>,
//! #   age: i32,
//! # }
//! impl StrunemixParsableData<'_, PersonAttrName> for PersonAttrData {
//!   fn from_name_and_data(name: PersonAttrName, data: &str) -> Result<Self, ()> {
//!     match name {
//!       PersonAttrName::Pseudo => Ok(PersonAttrData::Pseudo(data.to_string())),
//!       PersonAttrName::Phone => Ok(PersonAttrData::Phone(Some(data.to_string()))),
//!       PersonAttrName::Age => data.parse().map_err(|_| ()).map(|age| PersonAttrData::Age(age))
//!     }
//!   }
//! }
//! 
//! let pseudo_name = PersonAttrName::Pseudo;
//! let psudo_name_str = "pseudo";
//! let data = "MyCoolPseudo";
//! 
//! let pseudo_expected = PersonAttrData::Pseudo("MyCoolPseudo".to_string());
//! let pseudo_from_name = PersonAttrData::from_name_and_data(pseudo_name, data).unwrap();
//! let pseudo_from_str = PersonAttrData::from_str_and_data(psudo_name_str, data).unwrap();
//! 
//! assert_eq!(&pseudo_expected, &pseudo_from_name);
//! assert_eq!(&pseudo_expected, &pseudo_from_str);


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
/// #[derive(Default, Strunemix)]
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
/// #[derive(Default, Strunemix)]
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
/// #[derive(Default, Strunemix)]
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
pub use data::*;

mod name;
pub use name::*;

mod top;
pub use top::*;