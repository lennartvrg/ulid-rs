//! Diesel Ulid serialization and deserialization for the PostgreSQL backend.

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    query_builder::{bind_collector::RawBytesBindCollector, QueryId},
    serialize::{self, ToSql},
    sql_types::SqlType,
};
use uuid::Uuid;

/// The diesel Ulid dsl type
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 2950, array_oid = 2951))]
pub struct Ulid;

impl<DB> FromSql<Ulid, DB> for crate::Ulid
where
    DB: Backend<BindCollector = RawBytesBindCollector<DB>>,
    Uuid: FromSql<diesel::sql_types::Uuid, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> deserialize::Result<Self> {
        <Uuid as FromSql<diesel::sql_types::Uuid, DB>>::from_sql(bytes).map(Into::into)
    }
}

impl<DB: Backend> ToSql<Ulid, DB> for crate::Ulid
where
    DB: Backend<BindCollector = RawBytesBindCollector<DB>>,
    Uuid: ToSql<diesel::sql_types::Uuid, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, DB>) -> serialize::Result {
        <Uuid as ToSql<diesel::sql_types::Uuid, DB>>::to_sql(
            &Uuid::from_u128(self.0),
            &mut out.reborrow(),
        )
    }
}

#[derive(AsExpression, FromSqlRow)]
#[diesel(foreign_derive)]
#[diesel(sql_type = Ulid)]
#[allow(dead_code)]
struct UuidProxy(crate::Ulid);
