use crate::StrunemixName;

/// Trait implemented automatically on enums data generated by strunemix.
pub trait StrunemixData<T>
where 
    Self: Sized,
    T: StrunemixName + From<Self>
{
    /// Get the name of the enum value
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Default, Strunemix)]
    /// struct Person {
    ///   age: i32,
    ///   name: Option<String>
    /// }
    /// 
    /// let age = PersonAttrData::Age(42);
    /// 
    /// assert_eq!(age.name(), PersonAttrName::Age);
    fn name(self) -> T
    {
        T::from(self)
    }
}

/// Trait that must be implemented to allow the conversion from a string slice to the inner type of an enum data.
/// 
/// # Example
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
/// impl StrunemixParsableData<'_, PersonAttrName> for PersonAttrData {
///   fn from_name_and_data(name: PersonAttrName, data: &str) -> Result<Self, ()> {
///     match name {
///       PersonAttrName::Name => Ok(PersonAttrData::Name(Some(data.to_string()))),
///       PersonAttrName::Age => data.parse().map_err(|_| ()).map(|age| PersonAttrData::Age(age))
///     }
///   }
/// }
/// ```
/// This will make available [`StrunemixParsableData::from_str_and_data`] and [`StrunemixParsableData::from_name_and_data`].

pub trait StrunemixParsableData<'a, T>
where 
    Self: Sized,
    T: StrunemixName
{
    /// Make an enum data from a enum name and the associated data as a string slice.
    /// 
    /// ```rust
    /// # use strunemix::*;
    /// #
    /// # #[derive(Default, Strunemix)]
    /// # #[strunemix_derive_data(Debug, PartialEq)]
    /// # struct Person {
    /// #    age: i32,
    /// #    name: Option<String>,
    /// # }
    /// #
    /// # impl StrunemixParsableData<'_, PersonAttrName> for PersonAttrData {
    /// #  fn from_name_and_data(name: PersonAttrName, data: &str) -> Result<Self, ()> {
    /// #     match name {
    /// #       PersonAttrName::Name => Ok(PersonAttrData::Name(Some(data.to_string()))),
    /// #       PersonAttrName::Age => data.parse().map_err(|_| ()).map(|age| PersonAttrData::Age(age))
    /// #     }
    /// #   }
    /// # }
    /// #
    /// let age_name = PersonAttrName::Age;
    /// let age_data = "42";
    /// 
    /// let age = PersonAttrData::from_name_and_data(age_name, age_data).unwrap();
    /// 
    /// assert_eq!(PersonAttrData::Age(42), age);
    fn from_name_and_data(s: T, arg: &'a str) -> Result<Self, ()>;

    /// Make an enum data from a name as string and the associated data as a string slice.
    /// 
    /// ```rust
    /// # use strunemix::*;
    /// #
    /// # #[derive(Default, Strunemix)]
    /// # #[strunemix_derive_data(Debug, PartialEq)]
    /// # struct Person {
    /// #    age: i32,
    /// #    name: Option<String>,
    /// # }
    /// #
    /// # impl StrunemixParsableData<'_, PersonAttrName> for PersonAttrData {
    /// #  fn from_name_and_data(name: PersonAttrName, data: &str) -> Result<Self, ()> {
    /// #     match name {
    /// #       PersonAttrName::Name => Ok(PersonAttrData::Name(Some(data.to_string()))),
    /// #       PersonAttrName::Age => data.parse().map_err(|_| ()).map(|age| PersonAttrData::Age(age))
    /// #     }
    /// #   }
    /// # }
    /// #
    /// let age = PersonAttrData::from_str_and_data("age", "42").unwrap();
    /// 
    /// assert_eq!(PersonAttrData::Age(42), age);
    fn from_str_and_data(s: &str, arg: &'a str) -> Result<Self, ()> {
        match <T as StrunemixName>::from_str(s) {
            Some(name) => Self::from_name_and_data(name, arg),
            None => Err(())
        }
    }
}