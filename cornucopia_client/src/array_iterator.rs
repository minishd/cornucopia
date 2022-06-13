use fallible_iterator::FallibleIterator;
use postgres_protocol::types::{array_from_sql, ArrayValues};
use postgres_types::{FromSql, Kind, Type};
use std::{fmt::Debug, marker::PhantomData};

use crate::private::escape_domain;

pub struct ArrayIterator<'a, T: FromSql<'a>> {
    values: ArrayValues<'a>,
    ty: Type,
    _type: PhantomData<T>,
}

impl<'a, T: FromSql<'a>> Debug for ArrayIterator<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayIterator")
            .field("values", &"[T]")
            .field("ty", &self.ty)
            .field("_type", &self._type)
            .finish()
    }
}

impl<'a, T: FromSql<'a>> Iterator for ArrayIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.values
            .next()
            .unwrap()
            .map(|raw| T::from_sql_nullable(&self.ty, raw).unwrap())
    }
}

impl<'a, T: FromSql<'a>> FromSql<'a> for ArrayIterator<'a, T> {
    fn from_sql(
        ty: &Type,
        raw: &'a [u8],
    ) -> Result<ArrayIterator<'a, T>, Box<dyn std::error::Error + Sync + Send>> {
        let member_type = match *escape_domain(ty).kind() {
            Kind::Array(ref member) => escape_domain(member),
            _ => panic!("expected array type got {}", ty),
        };

        let array = array_from_sql(raw)?;
        if array.dimensions().count()? > 1 {
            return Err("array contains too many dimensions".into());
        }

        Ok(ArrayIterator {
            ty: member_type.to_owned(),
            values: array.values(),
            _type: PhantomData::default(),
        })
    }

    fn accepts(ty: &Type) -> bool {
        match *ty.kind() {
            Kind::Array(ref inner) => T::accepts(escape_domain(inner)),
            _ => false,
        }
    }
}
