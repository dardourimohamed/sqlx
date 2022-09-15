use crate::any::{Any, AnyValueRef};
use crate::arguments::Arguments;
use crate::encode::Encode;
use crate::types::Type;
use std::marker::PhantomData;

pub struct AnyArguments<'q> {
    values: AnyArgumentBuffer<'q>,
}

impl<'q, P> Arguments<'q> for AnyArguments<'q> {
    type Database = Any;

    fn reserve(&mut self, additional: usize, _size: usize) {
        self.values.reserve(additional);
    }

    fn add<T>(&mut self, value: T)
    where
        T: 'q + Send + Encode<'q, Self::Database> + Type<Self::Database>,
    {
        self.values.push(Box::new(value));
    }
}

pub struct AnyArgumentBuffer<'q>(pub(crate) Vec<AnyValueRef<'q>>);

impl<'q> Default for AnyArguments<'q> {
    fn default() -> Self {
        AnyArguments {
            values: AnyArgumentBuffer(vec![]),
        }
    }
}
