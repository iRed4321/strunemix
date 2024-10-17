use collect_array::CollectArrayResult;
use micromap::Map;

use crate::*;

#[doc(hidden)]
pub(crate) type StrunemixMap<T,U,const N: usize,A> = Map<T, (Option<U>, A), N>;

/// A form that allows to manipulate the struct data and some custom infos.
#[derive(Debug, PartialEq, Clone)]
pub struct StrunemixForm<T, U, const N: usize, A=()> 
where 
    T: StrunemixName + From<U> + PartialEq,
    U: StrunemixData<T>
{
    map: StrunemixMap<T,U,N,A>
}

impl<T,U,A, const N: usize> From<StrunemixMap<T,U,N,A>> for StrunemixForm<T, U, N, A> 
where 
    T: StrunemixName + From<U> + PartialEq,
    U: StrunemixData<T>
{
    fn from(map: StrunemixMap<T,U,N,A>) -> Self {
        Self {map}
    }
}

const ERR_MISSING_KEY: &str = "The key does not exist, unexpected error";

impl<T, U, const N: usize, A> StrunemixForm<T, U, N, A>
where 
    T: StrunemixName + From<U> + PartialEq,
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
    /// let foo_form_empty = Foo::empty_form::<AdditionalMetadata>();
    /// assert_eq!(foo_form_empty.get_data(FooAttrName::Bar), None);
    /// 
    /// let foo = Foo {bar: 42};
    /// let foo_form = foo.to_form::<AdditionalMetadata>();
    /// assert_eq!(foo_form.get_data(FooAttrName::Bar), Some(&FooAttrData::Bar(42)));
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn get_data(&self, name: impl QueryNameTrait<T>) -> Option<&U>{
        let name = name.to_attrname();
        self.map.get(&name).map(|(data, _)| data)
        .expect(ERR_MISSING_KEY)
        .as_ref()
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
    /// let form = Foo::empty_form::<AdditionalMetadata>();
    /// assert_eq!(form.get_info(FooAttrName::Bar), &AdditionalMetadata(String::new()));
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn get_info(&self, name: impl QueryNameTrait<T>) -> &A {
        let name = name.to_attrname();
        self.map.get(&name).map(|(_, info)| info)
        .expect(ERR_MISSING_KEY)
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
    /// let foo = Foo {bar: 42};
    /// let mut foo_form = foo.to_form::<AdditionalMetadata>();
    /// 
    /// if let Some(bar) = foo_form.get_data_mut(FooAttrName::Bar){
    ///     *bar = FooAttrData::Bar(666);
    /// }
    /// 
    /// assert_eq!(foo_form.get_data(FooAttrName::Bar), Some(&FooAttrData::Bar(666)));
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn get_data_mut(&mut self, name: impl QueryNameTrait<T>) -> Option<&mut U>{
        let name = name.to_attrname();
        self.map.get_mut(&name).map(|(data, _)| data)
        .expect(ERR_MISSING_KEY)
        .as_mut()
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
    /// let mut foo_form = Foo::empty_form::<AdditionalMetadata>();
    /// 
    /// if let AdditionalMetadata(bar) = foo_form.get_info_mut(FooAttrName::Bar){
    ///    bar.push_str("bar");
    /// }
    /// 
    /// assert_eq!(foo_form.get_info(FooAttrName::Bar), &AdditionalMetadata("bar".to_string()));
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn get_info_mut(&mut self, name: impl QueryNameTrait<T>) -> &mut A{
        let name = name.to_attrname();
        self.map.get_mut(&name).map(|(_, info)| info)
        .expect(ERR_MISSING_KEY)
    }

    /// Set the data of a field by its name
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
    /// let foo = Foo {bar: 42};
    /// let mut foo_form = foo.to_form::<AdditionalMetadata>();
    /// 
    /// foo_form.set_data(FooAttrName::Bar, FooAttrData::Bar(666));
    /// 
    /// assert_eq!(foo_form.get_data(FooAttrName::Bar), Some(&FooAttrData::Bar(666)));
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn set_data(&mut self, name: impl QueryNameTrait<T>, data: U){
        let name = name.to_attrname();
        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .0 = Some(data);
    }

    pub fn set_data_str<'a>(&mut self, name: impl QueryNameTrait<T>, data: &'a str)
    where
        T: StrunemixParsableData<'a, U>
    {
        let name = name.to_attrname();
        let data = name.add_data(data).unwrap();

        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .0 = Some(data);
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
    /// let foo = Foo {bar: 42};
    /// 
    /// let mut foo_form = foo.to_form::<AdditionalMetadata>();
    /// 
    /// foo_form.remove_data(FooAttrName::Bar);
    /// 
    /// assert_eq!(foo_form.get_data(FooAttrName::Bar), None);
    /// ```
    /// 
    /// # Panics
    /// Panics if the key does not exist, it should't happen.
    pub fn remove_data(&mut self, name: impl QueryNameTrait<T>){
        let name = name.to_attrname();
        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .0 = None;
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
    /// let mut foo_form = Foo::empty_form::<AdditionalMetadata>();
    /// 
    /// foo_form.set_info(FooAttrName::Bar, AdditionalMetadata("bar".to_string()));
    /// 
    /// assert_eq!(foo_form.get_info(FooAttrName::Bar), &AdditionalMetadata("bar".to_string()));
    /// 
    pub fn set_info(&mut self, name: impl QueryNameTrait<T>, info: A){
        let name = name.to_attrname();
        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .1 = info;
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
    /// let err = foo_form.clone().to_data_array().unwrap_err();
    /// assert_eq!(err, ());
    /// 
    /// foo_form.set_data(FooAttrName::Baz, FooAttrData::Baz(true));
    /// let data = foo_form.to_data_array().unwrap();
    /// assert_eq!(data, [FooAttrData::Bar(42), FooAttrData::Baz(true)]);
    /// ```
    pub fn to_data_array(self) -> Result<[U; N], ()>{

        if !self.is_complete(){
            return Err(());
        }

        let vec : CollectArrayResult<U,N> = self.map.into_iter()
        .map(|(_, (data, _))| data.unwrap())
        .collect();

        Ok(vec.unwrap())
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
    fn to_attrname(self) -> T;
}

impl<T> QueryNameTrait<T> for &str
where
    T: StrunemixName
{
    fn to_attrname(self) -> T {
        <T as StrunemixName>::from_str(self).unwrap()
    }
}

impl<T> QueryNameTrait<T> for T
where
    T: StrunemixName
{
    fn to_attrname(self) -> T {
        self
    }
}