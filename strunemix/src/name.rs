/// Trait implemented automatically on enums names generated by strunemix.
pub trait StrunemixName : Sized + std::str::FromStr
{

    /// Get the name of the enum value as a string slice
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
    /// let age = PersonAttrName::Age;
    /// 
    /// assert_eq!(age.get_str(), "age");
    /// ```
    fn get_str(&self) -> &'static str {
        panic!("This function should be implemented by the derive macro")
    }

    /// Get an enum value by its name
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Strunemix)]
    /// struct Person {
    ///   age: i32,
    ///   name: Option<String>,
    /// }
    /// 
    /// let age = PersonAttrName::from_str("age").unwrap();
    /// 
    /// assert_eq!(age, PersonAttrName::Age);
    fn from_str(name: &str) -> Option<Self> {
        <Self as std::str::FromStr>::from_str(name).ok()
    }

}

#[cfg(doc)]
use crate::StrunemixForm;

/// Trait that allow the conversion from a string slice to the inner type of an enum data.
/// 
/// Implement it on [StrunemixName] enum to allow the use of [add_data][StrunemixParsableData] on it.\
/// It also enables the use of [`StrunemixForm::set_data_str`] on a [StrunemixForm] to set data from a string slice.
/// 
/// # Example
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
/// impl StrunemixParsableData<'_, PersonAttrData> for PersonAttrName {
///   fn add_data(&self, data: &str) -> Result<PersonAttrData, ()> {
///     match &self {
///       PersonAttrName::Name => Ok(PersonAttrData::Name(Some(data.to_string()))),
///       PersonAttrName::Age => data.parse().map_err(|_| ()).map(|age| PersonAttrData::Age(age))
///     }
///   }
/// }
/// ```
pub trait StrunemixParsableData<'a, U> 
where 
    Self: StrunemixName
{
    fn add_data(&self, data: &'a str) -> Result<U, ()>;
}