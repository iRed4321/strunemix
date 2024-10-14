# strunemix

Strunemix allows to work with structs as if they were enums.

## Example

```rust
use strunemix::*;

#[derive(Default, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
struct Person {
   pseudo: String,
   phone: Option<String>,
   age: i32,
}

let person = Person {pseudo: "John".to_string(), phone: Some("123456789".to_string()), age: 42};

let pseudo_name = PersonAttrName::Pseudo;
let phone_name = PersonAttrName::Phone;
let age_name = PersonAttrName::Age;

assert_eq!(Person::as_attr_name_array(), [pseudo_name, phone_name, age_name]);
assert_eq!(pseudo_name.name(), "pseudo");
assert_eq!(phone_name.name(), "phone");
assert_eq!(age_name.name(), "age");

let pseudo_data = PersonAttrData::Pseudo("John".to_string());
let phone_data = PersonAttrData::Phone(Some("123456789".to_string()));
let age_data = PersonAttrData::Age(42);

assert_eq!(person.to_attr_data_array(), [pseudo_data, phone_data, age_data]);

```

License: MIT
