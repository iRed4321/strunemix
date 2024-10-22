use micromap::Map;

use crate::*;

#[doc(hidden)]
pub(crate) type StrunemixMap<T,U,const N: usize,A> = Map<T, (Option<U>, A), N>;

/// A form that allows to manipulate the struct data and some custom infos.
#[derive(Debug, PartialEq, Clone)]
pub struct StrunemixForm<T, U, const N: usize, A=()> 
where 
    T: StrunemixName + PartialEq,
    U: StrunemixData<T>
{
    map: StrunemixMap<T,U,N,A>
}

impl<T,U,A, const N: usize> From<StrunemixMap<T,U,N,A>> for StrunemixForm<T, U, N, A> 
where 
    T: StrunemixName + PartialEq,
    U: StrunemixData<T>
{
    fn from(map: StrunemixMap<T,U,N,A>) -> Self {
        Self {map}
    }
}

const ERR_MISSING_KEY: &str = "The key does not exist, unexpected error";

impl<T, U, const N: usize, A> StrunemixForm<T, U, N, A>
where 
    T: StrunemixName + PartialEq,
    U: StrunemixData<T>
{

    /// Get the data of a field by its name
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default)]
    /// struct AdditionalMetadata;
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Foo {
    ///    bar: i32,
    /// }
    /// 
    /// # fn main() -> Result<(), StrunemixError> {
    /// let foo_form_empty = Foo::empty_form::<AdditionalMetadata>();
    /// assert_eq!(foo_form_empty.get_data(FooAttrName::Bar)?, None);
    /// 
    /// let foo = Foo {bar: 42};
    /// let foo_form = foo.to_form::<AdditionalMetadata>();
    /// assert_eq!(foo_form.get_data(FooAttrName::Bar)?, Some(&FooAttrData::Bar(42)));
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn get_data(&self, name: impl QueryNameTrait<T>) -> Result<Option<&U>, StrunemixError>{
        let name = name.to_attrname()?;
        let found = self.map.get(&name).map(|(data, _)| data)
        .expect(ERR_MISSING_KEY)
        .as_ref();

        Ok(found)
    }

    /// Get the info of a field by its name
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default, PartialEq)]
    /// struct AdditionalMetadata(String);
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Foo {
    ///   bar: i32,
    /// }
    /// 
    /// # fn main() -> Result<(), StrunemixError> {
    /// let form = Foo::empty_form::<AdditionalMetadata>();
    /// assert_eq!(form.get_info(FooAttrName::Bar)?, &AdditionalMetadata(String::new()));
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn get_info(&self, name: impl QueryNameTrait<T>) -> Result<&A, StrunemixError> {
        let name = name.to_attrname()?;
        let found = self.map.get(&name).map(|(_, info)| info)
        .expect(ERR_MISSING_KEY);

        Ok(found)
    }

    /// Get a mutable reference to the data of a field by its name
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default)]
    /// struct AdditionalMetadata;
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Foo {
    ///   bar: i32,
    /// }
    /// 
    /// # fn main() -> Result<(), StrunemixError> {
    /// let foo = Foo {bar: 42};
    /// let mut foo_form = foo.to_form::<AdditionalMetadata>();
    /// 
    /// if let Some(bar) = foo_form.get_data_mut(FooAttrName::Bar)?{
    ///     *bar = FooAttrData::Bar(666);
    /// }
    /// 
    /// assert_eq!(foo_form.get_data(FooAttrName::Bar)?, Some(&FooAttrData::Bar(666)));
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn get_data_mut(&mut self, name: impl QueryNameTrait<T>) -> Result<Option<&mut U>, StrunemixError>{
        let name = name.to_attrname()?;
        let found = self.map.get_mut(&name).map(|(data, _)| data)
        .expect(ERR_MISSING_KEY)
        .as_mut();

        Ok(found)
    }

    /// Get a mutable reference to the info of a field by its name
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default, PartialEq)]
    /// struct AdditionalMetadata(String);
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Foo {
    ///   bar: i32,
    /// }
    /// 
    /// # fn main() -> Result<(), StrunemixError> {
    /// let mut foo_form = Foo::empty_form::<AdditionalMetadata>();
    /// 
    /// if let AdditionalMetadata(bar) = foo_form.get_info_mut(FooAttrName::Bar)?{
    ///    bar.push_str("bar");
    /// }
    /// 
    /// assert_eq!(foo_form.get_info(FooAttrName::Bar)?, &AdditionalMetadata("bar".to_string()));
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn get_info_mut(&mut self, name: impl QueryNameTrait<T>) -> Result<&mut A, StrunemixError>{
        let name = name.to_attrname()?;
        let found = self.map.get_mut(&name).map(|(_, info )| info)
        .expect(ERR_MISSING_KEY);

        Ok(found)
    }

    /// Set the data of a field by its name (string or enum) and the data enum value
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default)]
    /// struct AdditionalMetadata;
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Foo {
    ///   bar: i32,
    /// }
    /// 
    /// # fn main() -> Result<(), StrunemixError> {
    /// let foo = Foo {bar: 42};
    /// let mut foo_form = foo.to_form::<AdditionalMetadata>();
    /// 
    /// foo_form.set_data(FooAttrName::Bar, FooAttrData::Bar(666));
    /// //or
    /// foo_form.set_data("bar", FooAttrData::Bar(666));
    /// 
    /// assert_eq!(foo_form.get_data(FooAttrName::Bar)?, Some(&FooAttrData::Bar(666)));
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn set_data(&mut self, name: impl QueryNameTrait<T>, data: U) -> Result<(), StrunemixError> {
        let name = name.to_attrname()?;
        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .0 = Some(data);

        Ok(())
    }

    /// Set the data of a field from its name (string or enum) and a string data.
    /// Be sure to implement [StrunemixParsableData] on the enum name to allow the conversion from the string to the enum data and use this method.
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default)]
    /// struct AdditionalMetadata;
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Foo {
    ///   bar: i32,
    /// }
    /// 
    /// impl StrunemixParsableData<'_, FooAttrData> for FooAttrName {
    ///   fn add_data(&self, data: &str) -> Result<FooAttrData, StrunemixParseError> {
    ///     match self {
    ///       FooAttrName::Bar => Ok(FooAttrData::Bar(data.parse()?)),
    ///     }
    ///   }
    /// }
    /// 
    /// # fn main() -> Result<(), StrunemixError> {
    /// let foo = Foo {bar: 42};
    /// let mut foo_form = foo.to_form::<AdditionalMetadata>();
    /// 
    /// foo_form.set_data_str(FooAttrName::Bar, "666");
    /// //or
    /// foo_form.set_data_str("bar", "666");
    /// 
    /// assert_eq!(foo_form.get_data(FooAttrName::Bar)?, Some(&FooAttrData::Bar(666)));
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn set_data_str<'a>(&mut self, name: impl QueryNameTrait<T>, data: &'a str) -> Result<(), StrunemixError>
    where
        T: StrunemixParsableData<'a, U>
    {
        let name = name.to_attrname()?;
        let data = name.add_data(data)?;

        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .0 = Some(data);

        Ok(())
    }

    /// Remove the data of a field by its name
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default)]
    /// struct AdditionalMetadata;
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Foo {
    ///   bar: i32,
    /// }
    /// 
    /// # fn main() -> Result<(), StrunemixError> {
    /// let foo = Foo {bar: 42};
    /// 
    /// let mut foo_form = foo.to_form::<AdditionalMetadata>();
    /// 
    /// foo_form.remove_data(FooAttrName::Bar)?;
    /// 
    /// assert_eq!(foo_form.get_data(FooAttrName::Bar)?, None);
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn remove_data(&mut self, name: impl QueryNameTrait<T>) -> Result<(), StrunemixError> {
        let name = name.to_attrname()?;
        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .0 = None;

        Ok(())
    }

    /// Set the info of a field by its name
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default, PartialEq)]
    /// struct AdditionalMetadata(String);
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Foo {
    ///   bar: i32,
    /// }
    /// 
    /// # fn main() -> Result<(), StrunemixError> {
    /// let mut foo_form = Foo::empty_form::<AdditionalMetadata>();
    /// 
    /// foo_form.set_info(FooAttrName::Bar, AdditionalMetadata("bar".to_string()));
    /// 
    /// assert_eq!(foo_form.get_info(FooAttrName::Bar)?, &AdditionalMetadata("bar".to_string()));
    /// # Ok(())
    /// # }
    /// 
    pub fn set_info(&mut self, name: impl QueryNameTrait<T>, info: A) -> Result<(), StrunemixError> {
        let name = name.to_attrname()?;
        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .1 = info;

        Ok(())
    }

    /// Check that all the fields have data
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default)]
    /// struct AdditionalMetadata;
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Foo {
    ///   bar: i32,
    ///   baz: bool
    /// }
    /// 
    /// let mut foo_form = Foo::empty_form::<AdditionalMetadata>();
    /// 
    /// foo_form.set_data(FooAttrName::Bar, FooAttrData::Bar(42));
    /// assert_eq!(foo_form.is_complete(), false);
    /// 
    /// foo_form.set_data(FooAttrName::Baz, FooAttrData::Baz(true));
    /// assert_eq!(foo_form.is_complete(), true);
    /// 
    /// ```
    pub fn is_complete(&self) -> bool {
        self.map.iter().all(|(_, (data, _))| data.is_some())
    }

    /// Convert the form into an array of data if all the fields have data
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default, Clone)]
    /// struct AdditionalMetadata;
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq, Clone)]
    /// struct Foo {
    ///   bar: i32,
    ///   baz: bool
    /// }
    /// 
    /// let mut foo_form = Foo::empty_form::<AdditionalMetadata>();
    /// foo_form.set_data(FooAttrName::Bar, FooAttrData::Bar(42));
    /// 
    /// if let Err(StrunemixError::IncompleteForm) = foo_form.clone().to_data_array() {}
    /// else { assert!(false); }
    /// 
    /// foo_form.set_data(FooAttrName::Baz, FooAttrData::Baz(true));
    /// let data = foo_form.to_data_array().unwrap();
    /// assert_eq!(data, [FooAttrData::Bar(42), FooAttrData::Baz(true)]);
    /// ```
    pub fn to_data_array(self) -> Result<[U; N], StrunemixError>{

        if !self.is_complete(){
            return Err(StrunemixError::IncompleteForm);
        }

        let iter = self.map.into_iter().map(|(_, (data, _))| data.unwrap());

        let vec = array_init::from_iter(iter).unwrap();
        Ok(vec)
    }

    /// Convert the form into an array of info
    /// 
    /// ```rust
    /// use strunemix::*;
    /// 
    /// #[derive(Debug, Default, PartialEq)]
    /// struct AdditionalMetadata(String);
    /// 
    /// #[derive(Strunemix)]
    /// #[strunemix_derive_data(Debug, PartialEq)]
    /// struct Foo {
    ///   bar: i32,
    ///   baz: bool
    /// }
    /// 
    /// let mut foo_form = Foo::empty_form::<AdditionalMetadata>();
    /// 
    /// foo_form.set_info(FooAttrName::Bar, AdditionalMetadata("bar".to_string()));
    /// 
    /// let info = foo_form.get_info_array();
    /// 
    /// assert_eq!(info, [&AdditionalMetadata("bar".to_string()), &AdditionalMetadata(String::new())]);
    pub fn get_info_array(&self) -> Vec<&A>{
        self.map.iter().map(|(_, (_, info))| info).collect()
    }
}

#[doc(hidden)]
pub trait QueryNameTrait<T>
where 
    T: StrunemixName
{
    fn to_attrname(self) -> Result<T, StrunemixFromError>;
}

impl<T> QueryNameTrait<T> for &str
where
    T: StrunemixName
{
    fn to_attrname(self) -> Result<T, StrunemixFromError>
    {
        <T as name::StrunemixName>::from_str(self)
    }
}

impl<T> QueryNameTrait<T> for T
where
    T: StrunemixName
{
    fn to_attrname(self) -> Result<T, StrunemixFromError> {
        Ok(self)
    }
}