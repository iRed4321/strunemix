use strunemix::*;


#[derive(Debug, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
pub struct Person {
    name: String,
    #[strunemix(skip)]
    age: i32,
}

impl StrunemixParsableData<'_, PersonAttrName> for PersonAttrData {
    fn from_name_and_data(s: PersonAttrName, arg: &str) -> Result<Self, ()> {
        match s {
            PersonAttrName::Name => Ok(PersonAttrData::Name(arg.to_string()))
        }
    }
}

#[test]
fn from_name(){
    let name = PersonAttrName::Name;
    let data = "John";
    let name_data = PersonAttrData::from_name_and_data(name, data).unwrap();

    assert_eq!(name_data, PersonAttrData::Name("John".to_string()));
}

#[test]
fn from_string(){
    let name = "name";
    let data = "John";
    let name_data = PersonAttrData::from_str_and_data(name, data).unwrap();

    assert_eq!(name_data, PersonAttrData::Name("John".to_string()));
}