use crate::{StrunemixData, StrunemixName};

/// Trait implemented automatically on structs that have been strunemixed.
pub trait StrunemixTrait<T, U, const N: usize>
where 
    T: StrunemixName + From<U>
{

    /// The number of fields in the struct.
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Default, Strunemix)]
    /// struct Person {
    ///   age: i32,
    ///   name: Option<String>,
    /// }
    /// 
    /// assert_eq!(Person::FIELDS_COUNT, 2);
    ///
    const FIELDS_COUNT: usize = N;

    /// Convert a struct into an array of its attributes names, using enums of its field names.
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Default, Strunemix)]
    /// struct Person {
    ///    age: i32,
    ///    name: Option<String>,
    /// }
    /// 
    /// assert_eq!(Person::as_attr_name_array(), [PersonAttrName::Age, PersonAttrName::Name]);
    /// ```
    fn as_attr_name_array() -> [T; N] {
        panic!("This function should be implemented by the derive macro")
    }

    /// Convert a struct into an array of its attributes data, using enums variants of its field names.
    /// Note that the values are not cloned, they are moved, this function consumes the struct.
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Default, Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Person {
    ///    age: i32,
    ///    name: Option<String>,
    /// }
    /// 
    /// let person = Person {age: 42, name: Some("John".to_string())};
    /// 
    /// let age = PersonAttrData::Age(42);
    /// let name = PersonAttrData::Name(Some("John".to_string()));
    ///
    /// assert_eq!(person.to_attr_data_array(), [age, name]);
    /// ```
    fn to_attr_data_array(self) -> [U; N]
    where 
        U: StrunemixData<T>,
        Self: Sized
    {panic!("This function should be implemented by the derive macro")}

    /// Convert an array of attributes data into a struct, using enums variants of its field names.
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default, PartialEq, Strunemix)]
    /// struct Person {
    ///   pseudo: String,
    ///   phone: Option<String>,
    ///   age: i32,
    /// }
    /// 
    /// let data = [
    ///     PersonAttrData::Pseudo("John".to_string()),
    ///     PersonAttrData::Phone(Some("123456789".to_string())),
    ///     PersonAttrData::Age(42)
    /// ];
    /// 
    /// let person = Person::from_attr_data_array(data).unwrap();
    /// 
    /// let expected = Person {pseudo: "John".to_string(), phone: Some("123456789".to_string()), age: 42};
    /// 
    /// assert_eq!(person, expected);
    fn from_attr_data_array(data: [U; N]) -> Result<Self, ()>
    where Self: TryFrom<[U; N]> {
        TryFrom::try_from(data).map_err(|_| ())
    }
}