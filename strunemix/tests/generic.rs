use strunemix::*;


#[derive(Debug, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
pub struct Person<'a, A: Default>{
    name: Option<&'a str>,
    age: i32,
    note: A
}

impl<A> Default for Person<'_, A>
    where A: Default
{
    fn default() -> Self {
        Person {
            name: None,
            age: 0,
            note: A::default()
        }
    }
}

#[test]
fn fields_counts() {
    assert_eq!(Person::<String>::FIELDS_COUNT, 3);
}

#[test]
fn name_array() {

    let name = PersonAttrName::Name;
    let age = PersonAttrName::Age;
    let note = PersonAttrName::Note;

    assert_eq!(Person::<String>::as_attr_name_array(), [name, age, note]);
}

#[test]
fn data_array() {

    let person = Person::default();

    let name = PersonAttrData::<String>::Name(None);
    let age = PersonAttrData::<String>::Age(0);
    let note = PersonAttrData::<String>::Note(String::default());

    assert_eq!(person.to_attr_data_array(), [name, age, note]);
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
    let name = PersonAttrData::<String>::Name(Some("John"));
    assert_eq!(name.name(), PersonAttrName::Name);
}
