use jsonwebtoken::{decode, get_current_timestamp, Algorithm, DecodingKey, Validation};
use poem_openapi::SecurityScheme;
use serde::{de, Deserialize};

#[derive(SecurityScheme)]
#[oai(
    ty = "bearer",
    bearer_format = "Bearer <access_token>",
    checker = "bearer_checker"
)]
pub struct BearerAuth(pub User);

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub access_token: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Claims {
    email: String, // This is added by Auth0 Action (post-login)
    iss: String,
    #[serde(deserialize_with = "de_str_as_vec")]
    aud: Vec<String>,
    sub: String,
    azp: String,
    exp: u64,
    iat: u64,
    scope: String,
}

/// Deserialize a `String` into `Vec<String>` or just `Vec<String>`.
pub fn de_str_as_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrStringArray {
        String(String),
        StringArray(Vec<String>),
    }

    match StringOrStringArray::deserialize(deserializer)? {
        StringOrStringArray::String(s) => Ok(vec![s]),
        StringOrStringArray::StringArray(s) => Ok(s),
    }
}

pub async fn bearer_checker(
    req: &poem::Request,
    bearer: poem_openapi::auth::Bearer,
) -> Option<User> {
    tracing::debug!("Entered `bearer_checker` with {:?}", bearer.token);

    let config = req
        .data::<crate::config::Config>()
        .expect("Config not found in extensions");

    let token = decode::<Claims>(
        &bearer.token,
        &DecodingKey::from_rsa_pem(config.auth0_public_key.as_bytes()).ok()?,
        &Validation::new(Algorithm::RS256),
    )
    .map_err(|e| {
        tracing::debug!("Failed to decode token: {}", e);
        e
    })
    .ok()?;

    // If the issuer is not the same as the configured Auth0 domain, then the token is not valid.
    if config.auth0_domain.contains(token.claims.iss.as_str()) {
        tracing::debug!(
            "Token issuer ({}) does not match configured Auth0 domain ({})",
            token.claims.iss,
            config.auth0_domain
        );
        return None;
    }

    // Token cannot be issued in the future.
    if token.claims.iat >= get_current_timestamp() {
        tracing::debug!("Token was issued in the future");
        return None;
    }

    // Token cannot be used after it expires.
    if token.claims.exp <= get_current_timestamp() {
        tracing::debug!("Token has expired");
        return None;
    }

    // Token cannot be issued after it expires.
    if token.claims.iat >= token.claims.exp {
        tracing::debug!("Token was issued after it expired");
        return None;
    }

    // Token must have the `openid` and `email` scope.
    if !token.claims.scope.contains("openid") || !token.claims.scope.contains("email") {
        tracing::debug!("Token does not have the `openid` and `email` scope");
        return None;
    }

    let user = User {
        id: token.claims.sub,
        email: token.claims.email,
        access_token: bearer.token.clone(),
    };

    tracing::debug!("Exiting `bearer_checker`");

    Some(user)
}
