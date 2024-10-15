//! Strunemix allows to work with structs as if they were enums.
//! 
//! # Example
//! 
//! ```rust
//! use strunemix::*;
//! 
//! #[derive(Default, Strunemix)]
//! #[strunemix_derive_data(Debug, PartialEq)]
//! struct Person {
//!    pseudo: String,
//!    phone: Option<String>,
//!    age: i32,
//! }
//! 
//! let person = Person {pseudo: "John".to_string(), phone: Some("123456789".to_string()), age: 42};
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
//! let pseudo_data = PersonAttrData::Pseudo("John".to_string());
//! let phone_data = PersonAttrData::Phone(Some("123456789".to_string()));
//! let age_data = PersonAttrData::Age(42);
//! 
//! assert_eq!(person.to_attr_data_array(), [pseudo_data, phone_data, age_data]);
//! ```



/// Implements the [`StrunemixTrait`] on a struct automatically and generate the necessary enums and traits implementations.
/// 
/// # Examples
/// 
/// ## Skip a field
/// 
/// This attribute can be used to skip a field from the generated enums and traits.
/// 
/// ```rust
/// use strunemix::*;
/// 
/// #[derive(Default, Strunemix)]
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
/// ## Derive Macros on the enums names
/// 
/// This attribute can be used to derive some macros on the generated enums names.
/// If the attribute is not used, the macros Debug, PartialEq, Eq, Clone and Copy are derived by default.
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
/// This attribute can be used to derive some macros on the generated enums data.
/// If the attribute is not used, the macros Debug, PartialEq and Eq are derived by default.
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
/// This attribute can be used to derive some macros on both the generated enums names and data.
/// If the attribute is not used, the macros derived are the same as the ones derived on the enums names and data.
pub use strunemix_macros::Strunemix;

mod data;
pub use data::*;

mod name;
pub use name::*;

mod top;
pub use top::*;