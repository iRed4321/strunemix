use strunemix::*;

#[derive(Debug, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
#[strunemix_derive_name(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
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

    let person = Person {name: "John".to_string(), age: 42};

    let name = PersonAttrData::Name("John".to_string());
    let age = PersonAttrData::Age(42);

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

#[test]
fn form() {

    let person = Person {name: "John".to_string(), age: 42};

    let mut form = person.to_form::<String>();

    assert_eq!(form.get_data(PersonAttrName::Name).unwrap(), &PersonAttrData::Name("John".to_string()));
    assert_eq!(form.get_data(PersonAttrName::Age).unwrap(), &PersonAttrData::Age(42));

    let age = form.get_data_mut(PersonAttrName::Age).unwrap();
    if let PersonAttrData::Age(age) = age {
        *age = 43;
    }

    form.set_data(PersonAttrName::Name, PersonAttrData::Name("Jane".to_string()));

    form.set_info(PersonAttrName::Name, "Must be more than 3 characters".to_string());

    form.get_info_mut(PersonAttrName::Name).push_str(" and less than 10");

    let age = form.get_data(PersonAttrName::Age).unwrap();
    let name = form.get_data(PersonAttrName::Name).unwrap();
    let info_name = form.get_info(PersonAttrName::Name);

    
    assert_eq!(age, &PersonAttrData::Age(43));
    assert_eq!(name, &PersonAttrData::Name("Jane".to_string()));
    assert_eq!(info_name, "Must be more than 3 characters and less than 10");
    
    let age = form.get_data("age").unwrap();
    let name = form.get_data("name").unwrap();

    assert_eq!(age, &PersonAttrData::Age(43));
    assert_eq!(name, &PersonAttrData::Name("Jane".to_string()));

    let person_new = Person::from_form(form).unwrap();

    assert_eq!(person_new, Person {name: "Jane".to_string(), age: 43});

}

#[test]
fn form_empty() {

    let person = Person::empty_form::<()>();

    assert_eq!(person.get_data(PersonAttrName::Name), None);
    assert_eq!(person.get_data(PersonAttrName::Age), None);

    let finished = person.is_complete();
    assert_eq!(finished, false);

    let failed = Person::from_form(person).unwrap_err();

    assert_eq!(failed, ());
}