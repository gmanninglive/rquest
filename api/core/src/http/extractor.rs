use crate::{http::Error, AppState};
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::http::HeaderValue;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha384;
use time::OffsetDateTime;
use uuid::Uuid;

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);

const SCHEME_PREFIX: &str = "Bearer ";

/// Add this as a parameter to a handler function to require the user to be logged in.
///
/// Parses a JWT from the `Authorization: Token <token>` header.
pub struct AuthUser {
    pub id: Uuid,
}

/// Add this as a parameter to a handler function to optionally check if the user is logged in.
///
/// If the `Authorization` header is absent then this will be `Self(None)`, otherwise it will
/// validate the token.
///
/// This is in contrast to directly using `Option<AuthUser>`, which will be `None` if there
/// is *any* error in deserializing, which isn't exactly what we want.
pub struct MaybeAuthUser(pub Option<AuthUser>);

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthUserClaims {
    id: Uuid,
    /// Standard JWT `exp` claim.
    exp: i64,
}

impl AuthUser {
    pub fn to_jwt(&self, state: &AppState) -> String {
        let hmac = Hmac::<Sha384>::new_from_slice(state.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        AuthUserClaims {
            id: self.id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        }
        .sign_with_key(&hmac)
        .expect("HMAC signing should be infallible")
    }

    /// Attempt to parse `Self` from an `Authorization` header.
    fn from_authorization(state: &AppState, auth_header: &HeaderValue) -> Result<Self, Error> {
        let auth_header = auth_header.to_str().map_err(|_| {
            //log::debug!("Authorization header is not UTF-8");
            Error::Unauthorized
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            //log::debug!(
            //"Authorization header is using the wrong scheme: {:?}",
            //auth_header
            //);
            return Err(Error::Unauthorized);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];

        let jwt = jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token).map_err(
            |_| {
                //log::debug!(
                //"failed to parse Authorization header {:?}: {}",
                //auth_header,
                //e
                //);
                Error::Unauthorized
            },
        )?;

        let hmac = Hmac::<Sha384>::new_from_slice(state.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        // When choosing a JWT implementation, be sure to check that it validates that the signing
        // algorithm declared in the token matches the signing algorithm you're verifying with.
        // The `jwt` crate does.
        let jwt = jwt.verify_with_key(&hmac).map_err(|_| {
            //log::debug!("JWT failed to verify: {}", e);
            Error::Unauthorized
        })?;

        let (_header, claims) = jwt.into();

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            //log::debug!("token expired");
            return Err(Error::Unauthorized);
        }

        Ok(Self { id: claims.id })
    }
}

impl MaybeAuthUser {
    /// If this is `Self(Some(AuthUser))`, return `AuthUser::id`
    pub fn id(&self) -> Option<Uuid> {
        self.0.as_ref().map(|auth_user| auth_user.id)
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = Error;

    async fn from_request_parts(
        req: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Get the value of the `Authorization` header, if it was sent at all.
        let auth_header = req.headers.get(AUTHORIZATION).ok_or(Error::Unauthorized)?;

        Self::from_authorization(state, auth_header)
    }
}

#[async_trait]
impl FromRequestParts<AppState> for MaybeAuthUser {
    type Rejection = Error;

    async fn from_request_parts(
        req: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = req.headers.get(AUTHORIZATION).ok_or(Error::Unauthorized)?;
        Ok(Self(Some(AuthUser::from_authorization(
            state,
            auth_header,
        )?)))
    }
}
