use crate::core;
use crate::server::route::auth::provider::{
    oauth2_redirect, Oauth2CallbackQuery, Oauth2UrlResponse,
};
use crate::server::route::route_response_json;
use crate::server::{ConfigurationProviderOauth2, Data, Error, FromJsonValue, Oauth2Error};
use actix_identity::Identity;
use actix_web::http::{header, StatusCode};
use actix_web::{web, HttpResponse, ResponseError};
use futures::{future, Future};
use oauth2::curl::http_client;
use oauth2::{
    basic::BasicClient, AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use url::Url;

pub fn route_v1_scope() -> actix_web::Scope {
    web::scope("/microsoft").service(
        web::resource("/oauth2")
            .route(web::post().to_async(oauth2_request_handler))
            .route(web::get().to_async(oauth2_callback_handler)),
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct MicrosoftUser {
    mail: String,
}

fn oauth2_request_handler(
    data: web::Data<Data>,
    id: Identity,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let id = id.identity();

    web::block(move || request_inner(data.get_ref(), id))
        .map_err(Into::into)
        .then(route_response_json)
}

fn request_inner(data: &Data, id: Option<String>) -> Result<Oauth2UrlResponse, Error> {
    core::key::authenticate_service(data.driver(), id)
        .map_err(Into::into)
        .and_then(|service| microsoft_authorise(&data, &service).map_err(Into::into))
        .map(|url| Oauth2UrlResponse { url })
}

fn oauth2_callback_handler(
    data: web::Data<Data>,
    query: web::Query<serde_json::Value>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    Oauth2CallbackQuery::from_value(query.into_inner())
        .and_then(|query| {
            web::block(move || {
                let (service_id, access_token) =
                    microsoft_callback(data.get_ref(), &query.code, &query.state)?;
                Ok((data, service_id, access_token))
            })
            .map_err(Into::into)
        })
        .and_then(|(data, service_id, access_token)| {
            let email = microsoft_api_user_email(data.get_ref(), &access_token);
            let service_id = future::ok(service_id);
            let data = future::ok(data);
            data.join3(service_id, email)
        })
        .and_then(|(data, service_id, email)| {
            web::block(move || {
                core::auth::oauth2_login(
                    data.driver(),
                    service_id,
                    &email,
                    data.configuration().token_expiration_time(),
                )
                .map_err(Into::into)
            })
            .map_err(Into::into)
        })
        .then(|res| match res {
            Ok((service, token)) => future::ok(oauth2_redirect(service, token)),
            Err(err) => future::ok(err.error_response()),
        })
}

fn microsoft_authorise(data: &Data, service: &core::Service) -> Result<String, Error> {
    // Microsoft Graph supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorisation URL to redirect.
    let client = microsoft_client(data.configuration().provider_microsoft_oauth2())?;
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://graph.microsoft.com/User.Read".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    // Save the state and code verifier secrets as a CSRF key, value.
    core::csrf::create(
        data.driver(),
        service,
        &csrf_state.secret(),
        &pkce_code_verifier.secret(),
    )
    .map_err(Error::Core)?;

    Ok(authorize_url.to_string())
}

fn microsoft_callback(data: &Data, code: &str, state: &str) -> Result<(i64, String), Error> {
    // Read the CSRF key using state value, rebuild code verifier from value.
    let csrf = core::csrf::read_by_key(data.driver(), &state).map_err(Error::Core)?;
    let csrf = csrf.ok_or_else(|| Error::Oauth2(Oauth2Error::Csrf))?;

    // Exchange the code with a token.
    // TODO(refactor): Use async client.
    let client = microsoft_client(data.configuration().provider_microsoft_oauth2())?;
    let code = AuthorizationCode::new(code.to_owned());
    let pkce_code_verifier = PkceCodeVerifier::new(csrf.value);
    let token = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request(http_client)
        .map_err(|err| Error::Oauth2(Oauth2Error::Oauth2Request(err.into())))?;

    // Return access token value.
    Ok((csrf.service_id, token.access_token().secret().to_owned()))
}

fn microsoft_api_user_email(
    data: &Data,
    access_token: &str,
) -> impl Future<Item = String, Error = Error> {
    let client = actix_web::client::Client::new();
    let authorisation_header = format!("Bearer {}", access_token);
    client
        .get("https://graph.microsoft.com/v1.0/me")
        .header(header::AUTHORIZATION, authorisation_header)
        .header(header::CONTENT_TYPE, header::ContentType::json())
        .header(header::USER_AGENT, data.configuration().user_agent())
        .send()
        .map_err(|_err| Error::Oauth2(Oauth2Error::ActixClientSendRequest))
        .and_then(|res| {
            let status = res.status();
            match res.status() {
                StatusCode::OK => future::ok(res),
                _ => future::err(Error::Oauth2(Oauth2Error::StatusCode(status))),
            }
        })
        .and_then(|mut res| {
            res.json::<MicrosoftUser>()
                .map_err(|_err| Error::Oauth2(Oauth2Error::ActixPayload))
        })
        .map(|res| res.mail)
}

fn microsoft_client(provider: Option<&ConfigurationProviderOauth2>) -> Result<BasicClient, Error> {
    let provider = provider.ok_or(Error::Oauth2(Oauth2Error::Disabled))?;

    let graph_client_id = ClientId::new(provider.client_id.to_owned());
    let graph_client_secret = ClientSecret::new(provider.client_secret.to_owned());

    // Safe to unwrap here, known valid URLs.
    let auth_url =
        Url::parse("https://login.microsoftonline.com/common/oauth2/v2.0/authorize").unwrap();
    let auth_url = AuthUrl::new(auth_url);
    let token_url =
        Url::parse("https://login.microsoftonline.com/common/oauth2/v2.0/token").unwrap();
    let token_url = TokenUrl::new(token_url);

    let redirect_url = Url::parse(&provider.redirect_url).map_err(Error::UrlParse)?;
    Ok(BasicClient::new(
        graph_client_id,
        Some(graph_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_auth_type(AuthType::RequestBody)
    .set_redirect_url(RedirectUrl::new(redirect_url)))
}
