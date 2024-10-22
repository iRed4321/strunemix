use strunemix::*;

#[derive(Debug, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
pub struct Person {
    name: String,
    #[strunemix(skip)]
    age: i32,
}

#[test]
fn fields_counts() {
    assert_eq!(Person::FIELDS_COUNT, 1);
}

#[test]
fn name_array() {

    let name = PersonAttrName::Name;

    assert_eq!(Person::as_name_array(), [name]);
}

#[test]
fn data_array() {

    let person = Person {name: "John".to_string(), age: 42};

    let name = PersonAttrData::Name("John".to_string());

    assert_eq!(person.to_data_array(), [name]);
}

#[test]
fn from_enum_name() {
    let name = PersonAttrName::Name;
    assert!(name.get_str() == "name");
}

#[test]
fn make_enum_name() {
    let name = "name";

    let name_enum = PersonAttrName::from_str(name).unwrap();

    assert_eq!(name_enum, PersonAttrName::Name);

    let name_enum: PersonAttrName = name.parse().unwrap();

    assert_eq!(name_enum, PersonAttrName::Name);

}

#[test]
fn name_on_data() {
    let name = PersonAttrData::Name("John".to_string());
    assert_eq!(name.name(), PersonAttrName::Name);
}
