use strunemix::*;

#[derive(Debug, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
pub struct Person<'a, A>{
    name: Option<&'a str>,
    age: i32,
    note: A
}

impl<'a, 'b> StrunemixParsableData<'a, PersonAttrName> for PersonAttrData::<'b, String>
    where 'a: 'b
{
    fn from_name_and_data(s: PersonAttrName, arg: &'a str) -> Result<Self, ()>
    {
        match s {
            PersonAttrName::Name => Ok(PersonAttrData::Name(Some(arg))),
            PersonAttrName::Age => arg.parse().map_err(|_| ()).map(|age| PersonAttrData::Age(age)),
            PersonAttrName::Note => Ok(PersonAttrData::Note(arg.to_string()))
        }
    }
}

#[test]
fn from_name(){
    let name = PersonAttrName::Name;
    let data = "John";
    let name_data = PersonAttrData::from_name_and_data(name, data).unwrap();

    assert_eq!(name_data, PersonAttrData::Name(Some("John")));
}

#[test]
fn from_string(){
    let name = "name";
    let data = "John";
    let name_data = PersonAttrData::from_str_and_data(name, data).unwrap();

    assert_eq!(name_data, PersonAttrData::Name(Some("John")));
}