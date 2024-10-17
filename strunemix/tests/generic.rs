use strunemix::*;

#[derive(Debug, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
#[strunemix_derive_name(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Person<'a, A>{
    name: Option<&'a str>,
    age: i32,
    note: A
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

    let person = Person {name: Some("John"), age: 42, note: "note".to_string()};

    let name = PersonAttrData::<String>::Name(Some("John"));
    let age = PersonAttrData::<String>::Age(42);
    let note = PersonAttrData::<String>::Note("note".to_string());

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

#[test]
fn form() {

    let person = Person {name: Some("John"), age: 42, note: "note".to_string()};

    let mut form = person.to_form::<String>();

    assert_eq!(form.get_data(PersonAttrName::Name).unwrap(), &PersonAttrData::Name(Some("John")));
    assert_eq!(form.get_data(PersonAttrName::Age).unwrap(), &PersonAttrData::Age(42));

    let age = form.get_data_mut(PersonAttrName::Age).unwrap();
    if let PersonAttrData::Age(age) = age {
        *age = 43;
    }

    form.set_data(PersonAttrName::Name, PersonAttrData::Name(Some("Jane")));

    let age = form.get_data(PersonAttrName::Age).unwrap();
    let name = form.get_data(PersonAttrName::Name).unwrap();

    assert_eq!(age, &PersonAttrData::Age(43));
    assert_eq!(name, &PersonAttrData::Name(Some("Jane")));

    let person_new = Person::from_form(form).unwrap();

    assert_eq!(person_new, Person {name: Some("Jane"), age: 43, note: "note".to_string()});

}

#[test]
fn form_empty() {

    let person = Person::<String>::empty_form::<()>();

    assert_eq!(person.get_data(PersonAttrName::Name), None);
    assert_eq!(person.get_data(PersonAttrName::Age), None);

    let finished = person.is_complete();
    assert_eq!(finished, false);

    let failed = Person::from_form(person).unwrap_err();

    assert_eq!(failed, ());
}