use strunemix::*;

#[derive(Debug, PartialEq, Eq, Strunemix)]
#[strunemix_derive_data(Debug, PartialEq)]
pub struct Person1<'a, A>{
    name: Option<&'a str>,
    age: i32,
    note: A
}

#[test]
fn rebuild_normal<'a>() {
    let expected = Person1 {name: Some("John"), age: 42, note: "note".to_string()};

    let name = Person1AttrData::<String>::Name(Some("John"));
    let age = Person1AttrData::<String>::Age(42);
    let note = Person1AttrData::<String>::Note("note".to_string());

    let data = [name, age, note];

    let person = Person1::<String>::from_attr_data_array(data).unwrap();

    assert_eq!(person, expected);
}