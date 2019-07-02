use crate::client::sync_impl::SyncClient;
use crate::client::Error;
use crate::server::api::{
    route, AuthKeyBody, AuthKeyResponse, AuthLoginBody, AuthLoginResponse, AuthOauth2UrlResponse,
    AuthResetPasswordBody, AuthTokenBody, AuthTokenPartialResponse, AuthTokenResponse,
};
use actix_web::http::StatusCode;

impl SyncClient {
    pub fn auth_local_login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<AuthLoginResponse, Error> {
        let body = AuthLoginBody {
            email: email.to_owned(),
            password: password.to_owned(),
        };

        self.post_json(route::AUTH_LOCAL_LOGIN, &body)
            .send()
            .map_err(|_err| Error::Unwrap)
            .and_then(SyncClient::match_status_code)
            .and_then(|mut res| {
                res.json::<AuthLoginResponse>()
                    .map_err(|_err| Error::Unwrap)
            })
    }

    pub fn auth_local_reset_password(&self, email: &str) -> Result<(), Error> {
        let body = AuthResetPasswordBody {
            email: email.to_owned(),
            template: None,
        };

        self.post_json(route::AUTH_LOCAL_RESET_PASSWORD, &body)
            .send()
            .map_err(|_err| Error::Unwrap)
            .and_then(|res| match res.status() {
                StatusCode::OK => Ok(()),
                _ => Err(Error::Unwrap),
            })
    }

    pub fn auth_microsoft_oauth2_request(&self) -> Result<AuthOauth2UrlResponse, Error> {
        self.post(route::AUTH_MICROSOFT_OAUTH2)
            .send()
            .map_err(|_err| Error::Unwrap)
            .and_then(SyncClient::match_status_code)
            .and_then(|mut res| {
                res.json::<AuthOauth2UrlResponse>()
                    .map_err(|_err| Error::Unwrap)
            })
    }

    pub fn auth_key_verify(&self, key: &str) -> Result<AuthKeyResponse, Error> {
        let body = AuthKeyBody {
            key: key.to_owned(),
        };

        self.post_json(route::AUTH_KEY_VERIFY, &body)
            .send()
            .map_err(|_err| Error::Unwrap)
            .and_then(SyncClient::match_status_code)
            .and_then(|mut res| res.json::<AuthKeyResponse>().map_err(|_err| Error::Unwrap))
    }

    pub fn auth_key_revoke(&self, key: &str) -> Result<(), Error> {
        let body = AuthKeyBody {
            key: key.to_owned(),
        };

        self.post_json(route::AUTH_KEY_REVOKE, &body)
            .send()
            .map_err(|_err| Error::Unwrap)
            .and_then(SyncClient::match_status_code)
            .map(|_res| ())
    }

    pub fn auth_token_verify(&self, token: &str) -> Result<AuthTokenPartialResponse, Error> {
        let body = AuthTokenBody {
            token: token.to_owned(),
        };

        self.post_json(route::AUTH_TOKEN_VERIFY, &body)
            .send()
            .map_err(|_err| Error::Unwrap)
            .and_then(SyncClient::match_status_code)
            .and_then(|mut res| {
                res.json::<AuthTokenPartialResponse>()
                    .map_err(|_err| Error::Unwrap)
            })
    }

    pub fn auth_token_refresh(&self, token: &str) -> Result<AuthTokenResponse, Error> {
        let body = AuthTokenBody {
            token: token.to_owned(),
        };

        self.post_json(route::AUTH_TOKEN_REFRESH, &body)
            .send()
            .map_err(|_err| Error::Unwrap)
            .and_then(SyncClient::match_status_code)
            .and_then(|mut res| {
                res.json::<AuthTokenResponse>()
                    .map_err(|_err| Error::Unwrap)
            })
    }

    pub fn auth_token_revoke(&self, token: &str) -> Result<(), Error> {
        let body = AuthTokenBody {
            token: token.to_owned(),
        };

        self.post_json(route::AUTH_TOKEN_REVOKE, &body)
            .send()
            .map_err(|_err| Error::Unwrap)
            .and_then(SyncClient::match_status_code)
            .map(|_res| ())
    }
}
