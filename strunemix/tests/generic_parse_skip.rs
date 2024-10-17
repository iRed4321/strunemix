use strunemix::*;

#[derive(Debug, Default, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
#[strunemix_derive_name(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[strunemix_default]
pub struct Person<'a, A: Default>{
    name: Option<&'a str>,
    #[strunemix(skip)]
    age: i32,
    note: A
}


impl<'a, 'b> StrunemixParsableData<'a, PersonAttrData::<'b, String>> for PersonAttrName
    where 'a: 'b
{
    fn add_data(&self, data: &'a str) -> Result<PersonAttrData::<'b, String>, ()> {
        match self {
            PersonAttrName::Name => Ok(PersonAttrData::Name(Some(data))),
            PersonAttrName::Note => Ok(PersonAttrData::Note(data.to_string()))
        }
    }
}

#[test]
fn from_name(){
    let name = PersonAttrName::Name;
    let name_data = name.add_data("John").unwrap();

    assert_eq!(name_data, PersonAttrData::Name(Some("John")));
}

#[test]
fn from_string(){
    let name = "name";
    let data = "John";
    let name_data = PersonAttrName::from_str(name).unwrap().add_data(data).unwrap();

    assert_eq!(name_data, PersonAttrData::Name(Some("John")));
}

#[test]
fn form() {

    let person = Person {name: Some("John"), age: 42, note: "note".to_string()};

    let mut form = person.to_form::<String>();

    assert_eq!(form.get_data(PersonAttrName::Name).unwrap(), &PersonAttrData::Name(Some("John")));
    assert_eq!(form.get_data(PersonAttrName::Note).unwrap(), &PersonAttrData::Note("note".to_string()));

    let note = form.get_data_mut(PersonAttrName::Note).unwrap();
    if let PersonAttrData::Note(note) = note {
        *note = "new note".to_string();
    }

    form.set_data(PersonAttrName::Name, PersonAttrData::Name(Some("Jane")));

    let note = form.get_data(PersonAttrName::Note).unwrap();
    let name = form.get_data(PersonAttrName::Name).unwrap();

    assert_eq!(note, &PersonAttrData::Note("new note".to_string()));
    assert_eq!(name, &PersonAttrData::Name(Some("Jane")));

    let person_new = Person::from_form(form).unwrap();

    assert_eq!(person_new, Person {name: Some("Jane"), age: i32::default(), note: "new note".to_string()});

}