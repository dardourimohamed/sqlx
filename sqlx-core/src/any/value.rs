use std::borrow::Cow;
use std::marker::PhantomData;

use crate::any::error::mismatched_types;
use crate::any::{Any, AnyTypeInfo};
use crate::database::{Database, HasValueRef};
use crate::decode::Decode;
use crate::error::Error;
use crate::type_info::TypeInfo;
use crate::types::Type;
use crate::value::{Value, ValueRef};

#[non_exhaustive]
pub enum AnyValueKind {
    SmallInt(i16),
    Integer(i32),
    BigInt(i64),
    Real(f32),
    Double(f64),
    Text(String),
    Blob(Vec<u8>),
}

#[non_exhaustive]
pub enum AnyValueRefKind<'a> {
    SmallInt(i16),
    Integer(i32),
    BigInt(i64),
    Real(f32),
    Double(f64),
    Text(&'a str),
    Blob(&'a [u8]),
}

pub struct AnyValue {
    pub(crate) kind: AnyValueKind,
}

pub struct AnyValueRef<'a> {
    pub(crate) kind: AnyValueRefKind<'a>,
}

impl Value for AnyValue {
    type Database = Any;

    fn as_ref(&self) -> <Self::Database as HasValueRef<'_>>::ValueRef {
        AnyValueRef {
            kind: match &self.kind {
                AnyValueKind::SmallInt(&i) => AnyValueRefKind::SmallInt(i),
                AnyValueKind::Integer(&i) => AnyValueRefKind::Integer(i),
                AnyValueKind::BigInt(&i) => AnyValueRefKind::BigInt(i),
                AnyValueKind::Real(&r) => AnyValueRefKind::Real(r),
                AnyValueKind::Double(&d) => AnyValueRefKind::Double(d),
                AnyValueKind::Text(t) => AnyValueRefKind::Text(t),
                AnyValueKind::Blob(b) => AnyValueRefKind::Blob(t),
            },
        }
    }

    fn type_info(&self) -> Cow<'_, <Self::Database as Database>::TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        false
    }
}

impl<'a> ValueRef for AnyValueRef