// This file was generated with `cornucopia`. Do not modify.

pub mod types {
    pub mod public {
        #[derive(
            Debug, postgres_types::ToSql, postgres_types::FromSql, Clone, Copy, PartialEq, Eq,
        )]
        #[postgres(name = "spongebob_character")]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }

        #[derive(Debug, postgres_types::ToSql, Clone, PartialEq)]
        #[postgres(name = "custom_composite")]
        pub struct CustomComposite {
            pub wow: String,
            pub such_cool: i32,
            pub nice: super::public::SpongebobCharacter,
        }

        impl<'a> postgres_types::FromSql<'a> for CustomComposite {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                CustomComposite,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let wow = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let such_cool = postgres_types::private::read_value(fields[1].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let nice = postgres_types::private::read_value(fields[2].type_(), &mut buf)?;
                std::result::Result::Ok(CustomComposite {
                    wow,
                    such_cool,
                    nice,
                })
            }

            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_composite" && type_.schema() == "public"
            }
        }
        pub struct CustomCompositeBorrowed<'a> {
            pub wow: &'a str,
            pub such_cool: i32,
            pub nice: super::super::types::public::SpongebobCharacter,
        }

        impl<'a> postgres_types::FromSql<'a> for CustomCompositeBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                CustomCompositeBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let wow = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let such_cool = postgres_types::private::read_value(fields[1].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let nice = postgres_types::private::read_value(fields[2].type_(), &mut buf)?;
                std::result::Result::Ok(CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                })
            }

            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_composite" && type_.schema() == "public"
            }
        }

        impl<'a> From<CustomCompositeBorrowed<'a>> for CustomComposite {
            fn from(
                CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                }: CustomCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    wow: wow.into(),
                    such_cool,
                    nice,
                }
            }
        }

        #[derive(Debug, Clone, PartialEq, postgres_types::ToSql)]
        #[postgres(name = "custom_domain")]
        pub struct CustomDomain(pub Vec<super::super::types::public::CustomComposite>);

        impl<'a> postgres_types::FromSql<'a> for CustomDomain {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                CustomDomain,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let inner = match *_type.kind() {
                    postgres_types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(CustomDomain(postgres_types::private::read_value(
                    inner, &mut buf,
                )?))
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_domain" && type_.schema() == "public"
            }
        }
        pub struct CustomDomainBorrowed<'a>(
            pub  cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::CustomCompositeBorrowed<'a>,
            >,
        );

        impl<'a> postgres_types::FromSql<'a> for CustomDomainBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                CustomDomainBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let inner = match *_type.kind() {
                    postgres_types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(CustomDomainBorrowed(postgres_types::private::read_value(
                    inner, &mut buf,
                )?))
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "custom_domain" && type_.schema() == "public"
            }
        }

        impl<'a> From<CustomDomainBorrowed<'a>> for CustomDomain {
            fn from(CustomDomainBorrowed(inner): CustomDomainBorrowed<'a>) -> Self {
                Self(inner.map(|v| v.into()).collect())
            }
        }

        #[derive(Debug, Clone, PartialEq, postgres_types::ToSql)]
        #[postgres(name = "my_domain")]
        pub struct MyDomain(pub String);

        impl<'a> postgres_types::FromSql<'a> for MyDomain {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                MyDomain,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let inner = match *_type.kind() {
                    postgres_types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(MyDomain(postgres_types::private::read_value(
                    inner, &mut buf,
                )?))
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "my_domain" && type_.schema() == "public"
            }
        }
        pub struct MyDomainBorrowed<'a>(pub &'a str);

        impl<'a> postgres_types::FromSql<'a> for MyDomainBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                MyDomainBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let inner = match *_type.kind() {
                    postgres_types::Kind::Domain(ref inner) => inner,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                std::result::Result::Ok(MyDomainBorrowed(postgres_types::private::read_value(
                    inner, &mut buf,
                )?))
            }
            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "my_domain" && type_.schema() == "public"
            }
        }

        impl<'a> From<MyDomainBorrowed<'a>> for MyDomain {
            fn from(MyDomainBorrowed(inner): MyDomainBorrowed<'a>) -> Self {
                Self(inner.into())
            }
        }

        #[derive(Debug, postgres_types::ToSql, Clone, PartialEq)]
        #[postgres(name = "nightmare_composite")]
        pub struct NightmareComposite {
            pub custom: Vec<super::super::types::public::CustomComposite>,
            pub spongebob: Vec<super::super::types::public::SpongebobCharacter>,
        }

        impl<'a> postgres_types::FromSql<'a> for NightmareComposite {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                NightmareComposite,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let custom = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let spongebob = postgres_types::private::read_value(fields[1].type_(), &mut buf)?;
                std::result::Result::Ok(NightmareComposite { custom, spongebob })
            }

            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "nightmare_composite" && type_.schema() == "public"
            }
        }
        pub struct NightmareCompositeBorrowed<'a> {
            pub custom: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::CustomCompositeBorrowed<'a>,
            >,
            pub spongebob: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::SpongebobCharacter,
            >,
        }

        impl<'a> postgres_types::FromSql<'a> for NightmareCompositeBorrowed<'a> {
            fn from_sql(
                _type: &postgres_types::Type,
                buf: &'a [u8],
            ) -> std::result::Result<
                NightmareCompositeBorrowed<'a>,
                std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
            > {
                let fields = match *_type.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut buf = buf;
                let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let custom = postgres_types::private::read_value(fields[0].type_(), &mut buf)?;
                let _oid = postgres_types::private::read_be_i32(&mut buf)?;
                let spongebob = postgres_types::private::read_value(fields[1].type_(), &mut buf)?;
                std::result::Result::Ok(NightmareCompositeBorrowed { custom, spongebob })
            }

            fn accepts(type_: &postgres_types::Type) -> bool {
                type_.name() == "nightmare_composite" && type_.schema() == "public"
            }
        }

        impl<'a> From<NightmareCompositeBorrowed<'a>> for NightmareComposite {
            fn from(
                NightmareCompositeBorrowed { custom, spongebob }: NightmareCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    custom: custom.map(|v| v.into()).collect(),
                    spongebob: spongebob.map(|v| v.into()).collect(),
                }
            }
        }
    }
}

pub mod queries {
    pub mod module_2 {
        use futures::{StreamExt, TryStreamExt};

        pub struct AuthorNameStartingWithParams<'a> {
            pub start_str: &'a str,
        }
        impl<'a> AuthorNameStartingWithParams<'a> {
            pub fn query<C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith> {
                author_name_starting_with(client, &self.start_str)
            }
        }
        pub struct AuthorNameStartingWithBorrowed<'a> {
            pub name: &'a str,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorNameStartingWith {
            pub name: String,
        }
        impl<'a> From<AuthorNameStartingWithBorrowed<'a>> for AuthorNameStartingWith {
            fn from(
                AuthorNameStartingWithBorrowed { name }: AuthorNameStartingWithBorrowed<'a>,
            ) -> Self {
                Self { name: name.into() }
            }
        }
        pub struct AuthorNameStartingWithQuery<'a, C: cornucopia_client::GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 1],
            mapper: fn(AuthorNameStartingWithBorrowed) -> T,
        }

        impl<'a, C, T> AuthorNameStartingWithQuery<'a, C, T>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AuthorNameStartingWithBorrowed) -> R,
            ) -> AuthorNameStartingWithQuery<'a, C, R> {
                AuthorNameStartingWithQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }

            pub fn extractor(row: &tokio_postgres::row::Row) -> AuthorNameStartingWithBorrowed {
                AuthorNameStartingWithBorrowed { name: row.get(0) }
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    name
FROM
    Author
WHERE
    name LIKE CONCAT($1::text, '%');",
                    )
                    .await
            }

            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }

            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }

            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }

            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream.into_stream())
            }
        }
        pub fn author_name_starting_with<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            start_str: &'a &str,
        ) -> AuthorNameStartingWithQuery<'a, C, AuthorNameStartingWith> {
            AuthorNameStartingWithQuery {
                client,
                params: [start_str],
                mapper: |it| AuthorNameStartingWith::from(it),
            }
        }

        pub struct SelectEverythingBorrowed<'a> {
            pub custom_domain_: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::CustomCompositeBorrowed<'a>,
            >,
            pub custom_array_: cornucopia_client::ArrayIterator<
                'a,
                super::super::types::public::SpongebobCharacter,
            >,
            pub domain_: &'a str,
            pub array_: cornucopia_client::ArrayIterator<'a, bool>,
            pub bool_: bool,
            pub bool_opt: Option<bool>,
            pub boolean_: bool,
            pub char_: i8,
            pub smallint_: i16,
            pub int2_: i16,
            pub smallserial_: i16,
            pub serial2_: i16,
            pub int_: i32,
            pub int4_: i32,
            pub serial_: i32,
            pub serial4_: i32,
            pub bingint_: i64,
            pub int8_: i64,
            pub bigserial_: i64,
            pub serial8_: i64,
            pub float4_: f32,
            pub real_: f32,
            pub float8_: f64,
            pub double_precision_: f64,
            pub text_: &'a str,
            pub varchar_: &'a str,
            pub bytea_: &'a [u8],
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub date_: time::Date,
            pub time_: time::Time,
            pub json_: tokio_postgres::types::Json<&'a serde_json::value::RawValue>,
            pub jsonb_: tokio_postgres::types::Json<&'a serde_json::value::RawValue>,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectEverything {
            pub custom_domain_: Vec<super::super::types::public::CustomComposite>,
            pub custom_array_: Vec<super::super::types::public::SpongebobCharacter>,
            pub domain_: String,
            pub array_: Vec<bool>,
            pub bool_: bool,
            pub bool_opt: Option<bool>,
            pub boolean_: bool,
            pub char_: i8,
            pub smallint_: i16,
            pub int2_: i16,
            pub smallserial_: i16,
            pub serial2_: i16,
            pub int_: i32,
            pub int4_: i32,
            pub serial_: i32,
            pub serial4_: i32,
            pub bingint_: i64,
            pub int8_: i64,
            pub bigserial_: i64,
            pub serial8_: i64,
            pub float4_: f32,
            pub real_: f32,
            pub float8_: f64,
            pub double_precision_: f64,
            pub text_: String,
            pub varchar_: String,
            pub bytea_: Vec<u8>,
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub date_: time::Date,
            pub time_: time::Time,
            pub json_: tokio_postgres::types::Json<serde_json::Value>,
            pub jsonb_: tokio_postgres::types::Json<serde_json::Value>,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
        }
        impl<'a> From<SelectEverythingBorrowed<'a>> for SelectEverything {
            fn from(
                SelectEverythingBorrowed {
                    custom_domain_,
                    custom_array_,
                    domain_,
                    array_,
                    bool_,
                    bool_opt,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    smallserial_,
                    serial2_,
                    int_,
                    int4_,
                    serial_,
                    serial4_,
                    bingint_,
                    int8_,
                    bigserial_,
                    serial8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_,
                    varchar_,
                    bytea_,
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_,
                    jsonb_,
                    uuid_,
                    inet_,
                    macaddr_,
                }: SelectEverythingBorrowed<'a>,
            ) -> Self {
                Self {
                    custom_domain_: custom_domain_.map(|v| v.into()).collect(),
                    custom_array_: custom_array_.map(|v| v.into()).collect(),
                    domain_: domain_.into(),
                    array_: array_.map(|v| v.into()).collect(),
                    bool_,
                    bool_opt,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    smallserial_,
                    serial2_,
                    int_,
                    int4_,
                    serial_,
                    serial4_,
                    bingint_,
                    int8_,
                    bigserial_,
                    serial8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_: text_.into(),
                    varchar_: varchar_.into(),
                    bytea_: bytea_.into(),
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_: tokio_postgres::types::Json(
                        serde_json::from_str(json_.0.get()).unwrap(),
                    ),
                    jsonb_: tokio_postgres::types::Json(
                        serde_json::from_str(jsonb_.0.get()).unwrap(),
                    ),
                    uuid_,
                    inet_,
                    macaddr_,
                }
            }
        }
        pub struct SelectEverythingQuery<'a, C: cornucopia_client::GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
            mapper: fn(SelectEverythingBorrowed) -> T,
        }

        impl<'a, C, T> SelectEverythingQuery<'a, C, T>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectEverythingBorrowed) -> R,
            ) -> SelectEverythingQuery<'a, C, R> {
                SelectEverythingQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }

            pub fn extractor(row: &tokio_postgres::row::Row) -> SelectEverythingBorrowed {
                SelectEverythingBorrowed {
                    custom_domain_: row.get(0),
                    custom_array_: row.get(1),
                    domain_: row.get(2),
                    array_: row.get(3),
                    bool_: row.get(4),
                    bool_opt: row.get(5),
                    boolean_: row.get(6),
                    char_: row.get(7),
                    smallint_: row.get(8),
                    int2_: row.get(9),
                    smallserial_: row.get(10),
                    serial2_: row.get(11),
                    int_: row.get(12),
                    int4_: row.get(13),
                    serial_: row.get(14),
                    serial4_: row.get(15),
                    bingint_: row.get(16),
                    int8_: row.get(17),
                    bigserial_: row.get(18),
                    serial8_: row.get(19),
                    float4_: row.get(20),
                    real_: row.get(21),
                    float8_: row.get(22),
                    double_precision_: row.get(23),
                    text_: row.get(24),
                    varchar_: row.get(25),
                    bytea_: row.get(26),
                    timestamp_: row.get(27),
                    timestamp_without_time_zone_: row.get(28),
                    timestamptz_: row.get(29),
                    timestamp_with_time_zone_: row.get(30),
                    date_: row.get(31),
                    time_: row.get(32),
                    json_: row.get(33),
                    jsonb_: row.get(34),
                    uuid_: row.get(35),
                    inet_: row.get(36),
                    macaddr_: row.get(37),
                }
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
    custom_domain_,
    custom_array_,
    domain_,
    array_,
    bool_,
    bool_ AS bool_opt,
    boolean_,
    char_,
    smallint_,
    int2_,
    smallserial_,
    serial2_,
    int_,
    int4_,
    serial_,
    serial4_,
    bingint_,
    int8_,
    bigserial_,
    serial8_,
    float4_,
    real_,
    float8_,
    double_precision_,
    text_,
    varchar_,
    bytea_,
    timestamp_,
    timestamp_without_time_zone_,
    timestamptz_,
    timestamp_with_time_zone_,
    date_,
    time_,
    json_,
    jsonb_,
    uuid_,
    inet_,
    macaddr_
FROM
    Everything;",
                    )
                    .await
            }

            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }

            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }

            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }

            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream.into_stream())
            }
        }
        pub fn select_everything<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
        ) -> SelectEverythingQuery<'a, C, SelectEverything> {
            SelectEverythingQuery {
                client,
                params: [],
                mapper: |it| SelectEverything::from(it),
            }
        }

        pub struct InsertEverythingParams<'a> {
            pub custom_domain_: super::super::types::public::CustomDomain,
            pub custom_array_: &'a [super::super::types::public::SpongebobCharacter],
            pub domain_: super::super::types::public::MyDomain,
            pub array_: &'a [bool],
            pub bool_: bool,
            pub boolean_: bool,
            pub char_: i8,
            pub smallint_: i16,
            pub int2_: i16,
            pub smallserial_: i16,
            pub serial2_: i16,
            pub int_: i32,
            pub int4_: i32,
            pub serial_: i32,
            pub serial4_: i32,
            pub bingint_: i64,
            pub int8_: i64,
            pub bigserial_: i64,
            pub serial8_: i64,
            pub float4_: f32,
            pub real_: f32,
            pub float8_: f64,
            pub double_precision_: f64,
            pub text_: &'a str,
            pub varchar_: &'a str,
            pub bytea_: &'a [u8],
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub date_: time::Date,
            pub time_: time::Time,
            pub json_: tokio_postgres::types::Json<&'a serde_json::value::RawValue>,
            pub jsonb_: tokio_postgres::types::Json<&'a serde_json::value::RawValue>,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
        }
        impl<'a> InsertEverythingParams<'a> {
            pub fn query<C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> InsertEverythingQuery<'a, C> {
                insert_everything(
                    client,
                    &self.custom_domain_,
                    &self.custom_array_,
                    &self.domain_,
                    &self.array_,
                    &self.bool_,
                    &self.boolean_,
                    &self.char_,
                    &self.smallint_,
                    &self.int2_,
                    &self.smallserial_,
                    &self.serial2_,
                    &self.int_,
                    &self.int4_,
                    &self.serial_,
                    &self.serial4_,
                    &self.bingint_,
                    &self.int8_,
                    &self.bigserial_,
                    &self.serial8_,
                    &self.float4_,
                    &self.real_,
                    &self.float8_,
                    &self.double_precision_,
                    &self.text_,
                    &self.varchar_,
                    &self.bytea_,
                    &self.timestamp_,
                    &self.timestamp_without_time_zone_,
                    &self.timestamptz_,
                    &self.timestamp_with_time_zone_,
                    &self.date_,
                    &self.time_,
                    &self.json_,
                    &self.jsonb_,
                    &self.uuid_,
                    &self.inet_,
                    &self.macaddr_,
                )
            }
        }

        pub struct InsertEverythingQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 37],
        }

        impl<'a, C> InsertEverythingQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client.prepare("INSERT INTO Everything (custom_domain_, custom_array_, domain_, array_, bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37);

").await
            }

            pub async fn exec(self) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                self.client.execute(&stmt, &self.params).await
            }
        }
        pub fn insert_everything<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            custom_domain_: &'a super::super::types::public::CustomDomain,
            custom_array_: &'a &'a [super::super::types::public::SpongebobCharacter],
            domain_: &'a super::super::types::public::MyDomain,
            array_: &'a &'a [bool],
            bool_: &'a bool,
            boolean_: &'a bool,
            char_: &'a i8,
            smallint_: &'a i16,
            int2_: &'a i16,
            smallserial_: &'a i16,
            serial2_: &'a i16,
            int_: &'a i32,
            int4_: &'a i32,
            serial_: &'a i32,
            serial4_: &'a i32,
            bingint_: &'a i64,
            int8_: &'a i64,
            bigserial_: &'a i64,
            serial8_: &'a i64,
            float4_: &'a f32,
            real_: &'a f32,
            float8_: &'a f64,
            double_precision_: &'a f64,
            text_: &'a &str,
            varchar_: &'a &str,
            bytea_: &'a &[u8],
            timestamp_: &'a time::PrimitiveDateTime,
            timestamp_without_time_zone_: &'a time::PrimitiveDateTime,
            timestamptz_: &'a time::OffsetDateTime,
            timestamp_with_time_zone_: &'a time::OffsetDateTime,
            date_: &'a time::Date,
            time_: &'a time::Time,
            json_: &'a tokio_postgres::types::Json<&serde_json::value::RawValue>,
            jsonb_: &'a tokio_postgres::types::Json<&serde_json::value::RawValue>,
            uuid_: &'a uuid::Uuid,
            inet_: &'a std::net::IpAddr,
            macaddr_: &'a eui48::MacAddress,
        ) -> InsertEverythingQuery<'a, C> {
            InsertEverythingQuery {
                client,
                params: [
                    custom_domain_,
                    custom_array_,
                    domain_,
                    array_,
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    smallserial_,
                    serial2_,
                    int_,
                    int4_,
                    serial_,
                    serial4_,
                    bingint_,
                    int8_,
                    bigserial_,
                    serial8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_,
                    varchar_,
                    bytea_,
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_,
                    jsonb_,
                    uuid_,
                    inet_,
                    macaddr_,
                ],
            }
        }
    }

    pub mod module_1 {
        use futures::{StreamExt, TryStreamExt};

        pub struct InsertBookParams<'a> {
            pub book_name: &'a str,
        }
        impl<'a> InsertBookParams<'a> {
            pub fn query<C: cornucopia_client::GenericClient>(
                &'a self,
                client: &'a C,
            ) -> InsertBookQuery<'a, C> {
                insert_book(client, &self.book_name)
            }
        }

        pub struct InsertBookQuery<'a, C: cornucopia_client::GenericClient> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 1],
        }

        impl<'a, C> InsertBookQuery<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "INSERT INTO Book (title)
  VALUES ($1);",
                    )
                    .await
            }

            pub async fn exec(self) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                self.client.execute(&stmt, &self.params).await
            }
        }
        pub fn insert_book<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
            book_name: &'a &str,
        ) -> InsertBookQuery<'a, C> {
            InsertBookQuery {
                client,
                params: [book_name],
            }
        }

        pub struct NightmareBorrowed<'a> {
            pub composite: super::super::types::public::NightmareCompositeBorrowed<'a>,
            pub name: &'a str,
            pub names: cornucopia_client::ArrayIterator<'a, &'a str>,
            pub data: Option<&'a [u8]>,
            pub datas: Option<cornucopia_client::ArrayIterator<'a, &'a [u8]>>,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Nightmare {
            pub composite: super::super::types::public::NightmareComposite,
            pub name: String,
            pub names: Vec<String>,
            pub data: Option<Vec<u8>>,
            pub datas: Option<Vec<Vec<u8>>>,
        }
        impl<'a> From<NightmareBorrowed<'a>> for Nightmare {
            fn from(
                NightmareBorrowed {
                    composite,
                    name,
                    names,
                    data,
                    datas,
                }: NightmareBorrowed<'a>,
            ) -> Self {
                Self {
                    composite: composite.into(),
                    name: name.into(),
                    names: names.map(|v| v.into()).collect(),
                    data: data.map(|v| v.into()),
                    datas: datas.map(|v| v.map(|v| v.into()).collect()),
                }
            }
        }
        pub struct NightmareQuery<'a, C: cornucopia_client::GenericClient, T> {
            client: &'a C,
            params: [&'a (dyn tokio_postgres::types::ToSql + Sync); 0],
            mapper: fn(NightmareBorrowed) -> T,
        }

        impl<'a, C, T> NightmareQuery<'a, C, T>
        where
            C: cornucopia_client::GenericClient,
        {
            pub fn map<R>(self, mapper: fn(NightmareBorrowed) -> R) -> NightmareQuery<'a, C, R> {
                NightmareQuery {
                    client: self.client,
                    params: self.params,
                    mapper,
                }
            }

            pub fn extractor(row: &tokio_postgres::row::Row) -> NightmareBorrowed {
                NightmareBorrowed {
                    composite: row.get(0),
                    name: row.get(1),
                    names: row.get(2),
                    data: row.get(3),
                    datas: row.get(4),
                }
            }

            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
                self.client
                    .prepare(
                        "SELECT
  *
FROM
  nightmare;

",
                    )
                    .await
            }

            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                let row = self.client.query_one(&stmt, &self.params).await?;
                Ok((self.mapper)(Self::extractor(&row)))
            }

            pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.stream().await?.try_collect().await
            }

            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt().await?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }

            pub async fn stream(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>>,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt().await?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream.into_stream())
            }
        }
        pub fn nightmare<'a, C: cornucopia_client::GenericClient>(
            client: &'a C,
        ) -> NightmareQuery<'a, C, Nightmare> {
            NightmareQuery {
                client,
                params: [],
                mapper: |it| Nightmare::from(it),
            }
        }
    }
}
