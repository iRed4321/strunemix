# strunemix

Strunemix allows to work with structs as if they were enums.

## Example

```rust
use strunemix::*;

#[derive(Debug, PartialEq, Clone, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq, Clone)]
struct Person {
   pseudo: String,
   phone: Option<String>,
   age: i32,
}

let person = Person {pseudo: "MyCoolPseudo".to_string(), phone: Some("123456789".to_string()), age: 42};

let pseudo_name = PersonAttrName::Pseudo;
let phone_name = PersonAttrName::Phone;
let age_name = PersonAttrName::Age;

assert_eq!(Person::as_attr_name_array(), [pseudo_name, phone_name, age_name]);
assert_eq!(pseudo_name.name(), "pseudo");
assert_eq!(phone_name.name(), "phone");
assert_eq!(age_name.name(), "age");

let pseudo_data = PersonAttrData::Pseudo("MyCoolPseudo".to_string());
let phone_data = PersonAttrData::Phone(Some("123456789".to_string()));
let age_data = PersonAttrData::Age(42);

assert_eq!(person.clone().to_attr_data_array(), [pseudo_data.clone(), phone_data.clone(), age_data.clone()]);

let personcopy = Person::from_attr_data_array([pseudo_data, phone_data, age_data]).unwrap();

assert_eq!(person, personcopy);
```
If you want to build the attribute data from string values, you must implement the [`StrunemixParsableData`] trait to handle the conversion from the data to the struct fields.

```rust
impl StrunemixParsableData<'_, PersonAttrName> for PersonAttrData {
  fn from_name_and_data(name: PersonAttrName, data: &str) -> Result<Self, ()> {
    match name {
      PersonAttrName::Pseudo => Ok(PersonAttrData::Pseudo(data.to_string())),
      PersonAttrName::Phone => Ok(PersonAttrData::Phone(Some(data.to_string()))),
      PersonAttrName::Age => data.parse().map_err(|_| ()).map(|age| PersonAttrData::Age(age))
    }
  }
}

let pseudo_name = PersonAttrName::Pseudo;
let psudo_name_str = "pseudo";
let data = "MyCoolPseudo";

let pseudo_expected = PersonAttrData::Pseudo("MyCoolPseudo".to_string());
let pseudo_from_name = PersonAttrData::from_name_and_data(pseudo_name, data).unwrap();
let pseudo_from_str = PersonAttrData::from_str_and_data(psudo_name_str, data).unwrap();

assert_eq!(&pseudo_expected, &pseudo_from_name);
assert_eq!(&pseudo_expected, &pseudo_from_str);

License: MIT
