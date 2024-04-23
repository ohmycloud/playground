use std::ops::Deref;
use std::ops::DerefMut;
use std::vec::IntoIter;

fn main() {
    let mut wrapped_vec = WrapperStruct(vec![1,2,3]);
    wrapped_vec.iter().for_each(|v| println!("{}", v));
//    wrapped_vec.into_iter().for_each(|v| println!("{}", v));
    wrapped_vec.iter_mut().for_each(|v| println!("{}", v));
}

#[derive(Debug)]
struct WrapperStruct<T>(Vec<T>);

impl<T> Deref for WrapperStruct<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target { 
        &self.0
    }
}

impl<T> DerefMut for WrapperStruct<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { 
        &mut self.0
    }
}

impl<T> WrapperStruct<T> {
    fn into_iter(self) -> IntoIter<T> {
        self.0.into_iter()
    }
}
