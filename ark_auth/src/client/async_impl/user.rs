use crate::client::async_impl::AsyncClient;
use crate::client::Error;
use crate::server::route::user::{CreateBody, CreateResponse};
use futures::Future;

impl AsyncClient {
    pub fn user_create(
        &self,
        is_enabled: bool,
        name: &str,
        email: &str,
        password: Option<&str>,
    ) -> impl Future<Item = CreateResponse, Error = Error> {
        let body = CreateBody {
            is_enabled,
            name: name.to_owned(),
            email: email.to_owned(),
            password: password.map(String::from),
        };

        self.post("/v1/user")
            .send_json(&body)
            .map_err(|_err| Error::Unwrap)
            .and_then(AsyncClient::match_status_code)
            .and_then(|mut res| res.json::<CreateResponse>().map_err(|_err| Error::Unwrap))
    }
}
