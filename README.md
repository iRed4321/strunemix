# strunemix

[![Crates.io](https://img.shields.io/crates/v/strunemix.svg)](https://crates.io/crates/strunemix)
[![Docs](https://docs.rs/strunemix/badge.svg)](https://docs.rs/strunemix)

Strunemix allows to build a struct with a form of its fields, by deriving enums of them.

## Example

```rust
use strunemix::*;

#[derive(Debug, PartialEq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq, Clone)]
struct Person {
   pseudo: String,
   age: i32,
}

let person = Person {
    pseudo: "Elea".to_string(),
    age: 42
};

// Attributes names are easiliy turned from or into an array
assert_eq!(Person::as_name_array(), [PersonAttrName::Pseudo, PersonAttrName::Age]);
assert_eq!(PersonAttrName::Pseudo.get_str(), "pseudo");
assert_eq!(PersonAttrName::Age, "age".field_of::<Person>()?);

// Attributes data are turned to an enum
let pseudo_data = PersonAttrData::Pseudo("Elea".to_string());
let age_data = PersonAttrData::Age(42);
assert_eq!(person.to_data_array(), [pseudo_data, age_data]);

// Move between the struct to the form
let mut form = Person::empty_form::<()>();

// Set the data to the form
form.set_data(PersonAttrName::Pseudo, PersonAttrData::Pseudo("Anna".to_string()));
// or with the name as a string
form.set_data("age", PersonAttrData::Age(25));

// Get the data from the form
let anna = Person::from_form(form)?;

assert_eq!(anna, Person { pseudo: "Anna".to_string(), age: 25 });
```
If you want to build the attribute data from string values, you must implement the [`StrunemixParsableData`] trait to handle the conversion from the string data to the struct fields.

```rust
// Implement the trait for the enum names
impl StrunemixParsableData<'_, PersonAttrData> for PersonAttrName {
  fn add_data(&self, data: &str) -> Result<PersonAttrData, StrunemixParseError> {
    match self {
      PersonAttrName::Pseudo => Ok(PersonAttrData::Pseudo(data.to_string())),
      PersonAttrName::Age => Ok(PersonAttrData::Age(data.parse()?))
    }
  }
}

// Build the attribute data from string values
let pseudo_expected = PersonAttrData::Pseudo("Lois".to_string());
let pseudo = "pseudo".field_of::<Person>()?.add_data("Lois")?;
assert_eq!(&pseudo_expected, &pseudo);

let lois = Person { pseudo: "Lois".to_string(), age: 25 };
let mut form = lois.to_form::<()>();

// Add the data as a string, with the name as an enum or a string
form.set_data_str(PersonAttrName::Age, "42")?;
//or
form.set_data_str("age", "42")?;
```

License: MIT
