use strunemix::*;

#[derive(Debug, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
#[strunemix_derive_name(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Person1 {
    name: String,
    age: i32,
}

#[test]
fn rebuild_normal() {
    let expected = Person1 {name: "John".to_string(), age: 42};

    let name = Person1AttrData::Name("John".to_string());
    let age = Person1AttrData::Age(42);

    let data = [name, age];

    let person = Person1::from_attr_data_array(data).unwrap();

    assert_eq!(person, expected);
}

#[derive(Debug, Default, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
#[strunemix_derive_name(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[strunemix_default]
pub struct Person2 {
    name: String,
    #[strunemix(skip)]
    age: i32,
}

#[test]
fn rebuild_skipped_default() {
    use strunemix::*;
    
    let expected1 = Person2 {name: "John".to_string(), age: 0};
    let expected2 = Person2 {name: "John".to_string(), age: 42};

    let name = Person2AttrData::Name("John".to_string());

    let data = [name];

    let person = Person2::from_attr_data_array(data).unwrap();

    assert_eq!(person, expected1);
    assert_ne!(person, expected2);
}

#[test]
fn form() {

    let person = Person2 {name: "John".to_string(), age: 42};

    let mut form = person.to_form::<String>();

    assert_eq!(form.get_data(&Person2AttrName::Name).unwrap(), &Person2AttrData::Name("John".to_string()));

    form.set_data(&Person2AttrName::Name, Person2AttrData::Name("Jane".to_string()));

    let name = form.get_data(&Person2AttrName::Name).unwrap();

    assert_eq!(name, &Person2AttrData::Name("Jane".to_string()));

    let person_new = Person2::from_form(form).unwrap();

    assert_eq!(person_new, Person2 {name: "Jane".to_string(), age: i32::default()});

}