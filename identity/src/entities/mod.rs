use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct User {
    id: String,
    no: i32,
    email: String,
    email_verified: bool,
    name: String,
    username: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    gender: Option<Gender>,
    ic_number: Option<String>,
    phone_number: Option<String>,
    phone_number_verified: Option<bool>,
    nickname: Option<String>,
    avatar_url: Option<String>,
    address: Option<Address>,
    date_of_birth: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Ministry {
    id: String,
    name: String,
    description: String,
    department_id: String,
    team_id: String,
    satellite_id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct ConnectGroup {
    id: String,
    no: i32,
    name: String,
    variant: String,
    satellite_id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Enum, sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    #[oai(rename = "male")]
    Male,

    #[oai(rename = "female")]
    Female,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[serde(rename_all = "lowercase")]
pub struct Address {
    line_one: String,
    line_two: Option<String>,
    city: String,
    state: String,
    country: String,
    postal_code: String,
}

impl ::sqlx::encode::Encode<'_, ::sqlx::Postgres> for Address {
    fn encode_by_ref(
        &self,
        buf: &mut ::sqlx::postgres::PgArgumentBuffer,
    ) -> ::sqlx::encode::IsNull {
        let mut encoder = ::sqlx::postgres::types::PgRecordEncoder::new(buf);
        encoder.encode(&self.line_one);
        encoder.encode(&self.line_two);
        encoder.encode(&self.city);
        encoder.encode(&self.state);
        encoder.encode(&self.country);
        encoder.encode(&self.postal_code);
        encoder.finish();
        ::sqlx::encode::IsNull::No
    }
    fn size_hint(&self) -> ::std::primitive::usize {
        6usize * (4 + 4)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.line_one)
            + <Option<String> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.line_two,
            )
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.city)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.state)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.country)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.postal_code)
    }
}

impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Address {
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let line_one = decoder.try_decode::<String>()?;
        let line_two = decoder.try_decode::<Option<String>>()?;
        let city = decoder.try_decode::<String>()?;
        let state = decoder.try_decode::<String>()?;
        let country = decoder.try_decode::<String>()?;
        let postal_code = decoder.try_decode::<String>()?;
        ::std::result::Result::Ok(Address {
            line_one,
            line_two,
            city,
            state,
            country,
            postal_code,
        })
    }
}

impl ::sqlx::Type<::sqlx::Postgres> for Address {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("_address")
    }
}
