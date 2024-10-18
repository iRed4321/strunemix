use strunemix::*;

#[derive(Debug, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
pub struct Person<'a, A>{
    name: Option<&'a str>,
    age: i32,
    note: A
}

impl<'a, 'b> StrunemixParsableData<'a, PersonAttrData::<'b, String>> for PersonAttrName
    where 'a: 'b
{
    fn add_data(&self, data: &'a str) -> Result<PersonAttrData::<'b, String>, StrunemixParseError> {
        match self {
            PersonAttrName::Name => Ok(PersonAttrData::Name(Some(data))),
            PersonAttrName::Age => Ok(data.parse().map(|age| PersonAttrData::Age(age))?),
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