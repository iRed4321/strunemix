use strunemix::*;


#[derive(Debug, Default, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
pub struct Person {
    name: String,
    age: i32,
}

#[test]
fn fields_counts() {
    assert_eq!(Person::FIELDS_COUNT, 2);
}

#[test]
fn name_array() {

    let name = PersonAttrName::Name;
    let age = PersonAttrName::Age;

    assert_eq!(Person::as_attr_name_array(), [name, age]);
}

#[test]
fn data_array() {

    let person = Person::default();

    let name = PersonAttrData::Name("".to_string());
    let age = PersonAttrData::Age(0);

    assert_eq!(person.to_attr_data_array(), [name, age]);
}

#[test]
fn from_enum_name() {
    let age = PersonAttrName::Age;
    assert!(age.name() == "age");
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