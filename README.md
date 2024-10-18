# strunemix

[![Crates.io](https://img.shields.io/crates/v/strunemix.svg)](https://crates.io/crates/strunemix)
[![Docs](https://docs.rs/strunemix/badge.svg)](https://docs.rs/strunemix)

Strunemix allows to build a struct with a form of its fields, by deriving enums of them.

## Example

```rust
use strunemix::*;

#[derive(Debug, PartialEq, Clone, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq, Clone)]
struct Person {
   pseudo: String,
   age: i32,
}

let person = Person {
    pseudo: "BestPseudo".to_string(),
    age: 42
};

// Attributes names are turned to an enum
assert_eq!(Person::as_attr_name_array(), [PersonAttrName::Pseudo, PersonAttrName::Age]);
assert_eq!(PersonAttrName::Pseudo.name(), "pseudo");
assert_eq!(PersonAttrName::Age.name(), "age");

// Attributes data are turned to an enum
let pseudo_data = PersonAttrData::Pseudo("BestPseudo".to_string());
let age_data = PersonAttrData::Age(42);
assert_eq!(person.to_attr_data_array(), [pseudo_data, age_data]);

// Move between the struct and a form of it
let mut form = Person::empty_form::<()>();
form.set_data(PersonAttrName::Pseudo, PersonAttrData::Pseudo("BeckyTheBest".to_string()));
form.set_data(PersonAttrName::Age, PersonAttrData::Age(25));

let becky = Person::from_form(form).unwrap();

assert_eq!(becky, Person { pseudo: "BeckyTheBest".to_string(), age: 25 });
```
If you want to build the attribute data from string values, you must implement the [`StrunemixParsableData`] trait to handle the conversion from the string data to the struct fields.

```rust
// Implement the trait for the enum names
impl StrunemixParsableData<'_, PersonAttrData> for PersonAttrName {
  fn add_data(&self, data: &str) -> Result<PersonAttrData, ()> {
    match self {
      PersonAttrName::Pseudo => Ok(PersonAttrData::Pseudo(data.to_string())),
      PersonAttrName::Age => data.parse().map_err(|_| ()).map(|age| PersonAttrData::Age(age))
    }
  }
}

// Build the attribute data from string values
let pseudo_expected = PersonAttrData::Pseudo("MyCoolPseudo".to_string());
let pseudo = PersonAttrName::from_str("pseudo").unwrap().add_data("MyCoolPseudo").unwrap();
assert_eq!(&pseudo_expected, &pseudo);

let mut form = Person::empty_form::<()>();

// Add the data to the form the way you want

// With the generated enums
form.set_data(PersonAttrName::Age, PersonAttrData::Age(42));

// or with the name as a string
form.set_data("age", PersonAttrData::Age(42));

// or with a string for the data
form.set_data_str(PersonAttrName::Pseudo, "MyCoolPseudo");

// or with only strings
form.set_data_str("pseudo", "MyCoolPseudo");

```

License: MIT
