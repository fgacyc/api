use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub no: i32,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub username: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub gender: Option<Gender>,
    pub ic_number: Option<String>,
    pub phone_number: Option<String>,
    pub phone_number_verified: Option<bool>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub address: Option<Address>,
    pub date_of_birth: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Ministry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub department_id: String,
    pub team_id: String,
    pub satellite_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct PastoralRole {
    id: String,
    name: String,
    description: String,
    weight: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct ConnectGroup {
    pub id: String,
    pub no: i32,
    pub name: String,
    pub variant: String,
    pub satellite_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct MinistryRole {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weight: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Satellite {
    pub id: String,
    pub no: i32,
    pub name: String,
    pub address: Address,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct MinistryTeam {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct MinistryDepartment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Enum, sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
#[oai(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[serde(rename_all = "lowercase")]
pub struct Address {
    pub line_one: String,
    pub line_two: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
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
