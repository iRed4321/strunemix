use std::borrow::Cow;

use crate::*;

/// Trait implemented automatically on structs that have been strunemixed.
pub trait StrunemixTrait<T, U, const N: usize>
where 
    T: StrunemixName + From<U>,
    U: StrunemixData<T>
{

    /// The number of fields in the struct.
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Strunemix)]
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
    /// #[derive(Strunemix)]
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
    /// #[derive(Strunemix)]
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
    /// #[derive(Debug, PartialEq, Strunemix)]
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

    /// Consume the struct into a map-like structure convienient for form handling.
    /// You have to provide an associated type for the form data. This can be useful to store the form-specific metadata.
    /// The created form will initialize this associated type with a default value, so the [`Default`] trait must be implemented for it.
    fn to_form<A>(self) -> StrunemixForm<T, U, N, A>
    where 
        A: Default,
        T: Eq,
        Self: Sized,
    {
        let names = Self::as_attr_name_array();
        let datas = self.to_attr_data_array();

        let res: StrunemixMap<T, U, N, A> = names.into_iter().zip(datas.into_iter())
        .map(|(name, data)| (name, (Some(data), A::default())))
        .collect();

        StrunemixForm::from(res)
    }

    /// Consume a form and convert it into a struct.
    fn from_form<A>(form: StrunemixForm<T, U, N, A>) -> Result<Self, ()>
    where
        Self: TryFrom<[U; N]>,
        T: PartialEq
    {
        let datas = form.to_data_array()?;
        let datas = datas.try_into().map_err(|_| ())?;
        
        Self::from_attr_data_array(datas)
    }

    /// Create an empty form with default values for the associated type.
    fn empty_form<A>() -> StrunemixForm<T, U, N, A>
    where 
        A: Default,
        T: PartialEq
    {
        let names = Self::as_attr_name_array();

        let res: StrunemixMap<T, U, N, A> = names.into_iter()
        .map(|name| (name, (None, A::default())))
        .collect();

        StrunemixForm::from(res)
    }

}

/// Trait implemented automatically to convert a string into an enum name easily.
/// 
/// ```rust
/// use strunemix::*;
/// 
/// #[derive(Debug, PartialEq, Strunemix)]
/// struct Foo {
///    bar: i32,
/// }
/// 
/// let name = "bar".field_of::<Foo>().unwrap();
/// assert_eq!(name, FooAttrName::Bar);
pub trait AsEnumName<T, U, const N: usize>
{
    #[doc(hidden)]
    fn field_of<S>(&self)-> Option<T>
    where
        S: StrunemixTrait<T, U, N>,
        T: StrunemixName + From<U>,
        U: StrunemixData<T>;
}

impl<T, U, const N: usize> AsEnumName<T, U, N> for &str
{
    fn field_of<S>(&self)-> Option<T>
    where
        S: StrunemixTrait<T, U, N>,
        T: StrunemixName + From<U>,
        U: StrunemixData<T>{
            <T as StrunemixName>::from_str(self)
        }
}

impl<T, U, const N: usize> AsEnumName<T, U, N> for Cow<'_, str>
{
    fn field_of<S>(&self)-> Option<T>
    where
        S: StrunemixTrait<T, U, N>,
        T: StrunemixName + From<U>,
        U: StrunemixData<T>{
            <T as StrunemixName>::from_str(self)
        }
}