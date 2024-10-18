use strunemix::*;


#[derive(Debug, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
pub struct Person {
    name: String,
    #[strunemix(skip)]
    age: i32,
}

impl<'a> StrunemixParsableData<'a, PersonAttrData> for PersonAttrName {
    fn add_data(&self, data: &str) -> Result<PersonAttrData, StrunemixParseError> {
        match &self {
            PersonAttrName::Name => Ok(PersonAttrData::Name(data.to_string())),
        }
    }
}

#[test]
fn from_name(){
    let name = PersonAttrName::Name;

    let name_data = name.add_data("John").unwrap();
    assert_eq!(name_data, PersonAttrData::Name("John".to_string()));
}

#[test]
fn from_string(){
    let name = "name";
    let data = "John";
    let name_data = PersonAttrName::from_str(name).unwrap().add_data(data).unwrap();

    assert_eq!(name_data, PersonAttrData::Name("John".to_string()));
}