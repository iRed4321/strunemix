use std::collections::BTreeMap;

use crate::*;

#[derive(Debug)]
pub struct StrunemixForm<A, T, U> 
where 
    T: StrunemixName + From<U> + Ord,
    U: StrunemixData<T>
{
    map: BTreeMap<T,(Option<U>,A)>
}

impl<T,U,A> From<BTreeMap<T,(Option<U>,A)>> for StrunemixForm<A, T, U> 
where 
    T: StrunemixName + From<U> + Ord,
    U: StrunemixData<T>
{
    fn from(map: BTreeMap<T,(Option<U>,A)>) -> Self {
        Self {map}
    }
}

const ERR_MISSING_KEY: &str = "The key does not exist, unexpected error";

impl<A, T, U> StrunemixForm<A, T, U> 
where 
    T: StrunemixName + From<U> + Ord,
    U: StrunemixData<T>
{

    pub fn get_data(&self, name: T) -> Option<&U>{
        self.map.get(&name).map(|(data, _)| data)
        .expect(ERR_MISSING_KEY)
        .as_ref()
    }

    pub fn get_info(&self, name: T) -> &A {
        self.map.get(&name).map(|(_, info)| info)
        .expect(ERR_MISSING_KEY)
    }

    pub fn get_data_mut(&mut self, name: T) -> Option<&mut U>{
        self.map.get_mut(&name).map(|(data, _)| data)
        .expect(ERR_MISSING_KEY)
        .as_mut()
    }

    pub fn get_info_mut(&mut self, name: T) -> &mut A{
        self.map.get_mut(&name).map(|(_, info)| info)
        .expect(ERR_MISSING_KEY)
    }

    pub fn set_data(&mut self, name: T, data: U){
        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .0 = Some(data);
    }

    pub fn remove_data(&mut self, name: T){
        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .0 = None;
    }

    pub fn set_info(&mut self, name: T, info: A){
        self.map.get_mut(&name)
        .expect(ERR_MISSING_KEY)
        .1 = info;
    }

    pub fn is_complete(&self) -> bool {
        self.map.iter().all(|(_, (data, _))| data.is_some())
    }

    pub fn to_data_array(self) -> Result<Vec<U>, ()> {
        self.map.into_iter()
        .map(|(_, (data, _))| data.ok_or(()))
        .collect()
    }

    pub fn get_info_array(&self) -> Vec<&A>{
        self.map.iter().map(|(_, (_, info))| info).collect()
    }
}