use crate::internal::*;

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestOauth2Authorize {
    #[serde(default)]
    #[validate(custom = "validate::csrf_token")]
    pub csrf_token: String,
    #[serde(default)]
    #[validate(length(min = 1, max = 20))]
    pub auth_type: String,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(custom = "validate::password")]
    pub password: Option<String>,
    #[validate(custom = "validate::oauth2_provider")]
    pub oauth2_provider: Option<String>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestOauth2AuthorizeQuery {
    #[serde(default)]
    #[validate(length(min = 1, max = 10))]
    pub response_type: String,
    #[serde(default)]
    #[validate(custom = "validate::client_id")]
    pub client_id: String,
    #[serde(default)]
    #[validate(url)]
    pub redirect_uri: String,
    #[serde(default)]
    #[validate(custom = "validate::state")]
    pub state: String,
    #[validate(custom = "validate::scope")]
    pub scope: Option<String>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestOauth2Introspect {
    #[serde(default)]
    #[validate(custom = "validate::token")]
    pub token: String,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestOauth2RedirectQuery {
    #[serde(default)]
    #[validate(custom = "validate::code")]
    pub code: String,
    #[serde(default)]
    #[validate(custom = "validate::state")]
    pub state: String,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestOauth2Token {
    #[serde(default)]
    #[validate(length(min = 1, max = 20))]
    pub grant_type: String,
    #[validate(custom = "validate::code")]
    pub code: Option<String>,
    #[validate(url)]
    pub redirect_uri: Option<String>,
    #[validate(custom = "validate::token")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAuthDeleteQuery {
    pub client_id: Option<Uuid>,
    #[validate(url)]
    pub redirect_uri: Option<String>,
    #[validate(length(min = 1, max = 10))]
    pub response_type: Option<String>,
    #[validate(custom = "validate::code")]
    pub code: Option<String>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAuthDelete {
    #[serde(default)]
    #[validate(custom = "validate::csrf_token")]
    pub csrf_token: String,
    #[serde(default)]
    #[validate(custom = "validate::password")]
    pub password: String,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAuthEmailUpdate {
    #[serde(default)]
    #[validate(custom = "validate::csrf_token")]
    pub csrf_token: String,
    #[serde(default)]
    #[validate(custom = "validate::password")]
    pub password: String,
    #[serde(default)]
    #[validate(email)]
    pub email_new: String,
    #[serde(default)]
    #[validate(email)]
    pub email_confirm: String,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAuthQuery {
    pub client_id: Uuid,
    #[serde(default)]
    #[validate(url)]
    pub redirect_uri: String,
    #[validate(length(min = 1, max = 500))]
    pub message: Option<String>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAuthPasswordReset {
    #[serde(default)]
    #[validate(custom = "validate::csrf_token")]
    pub csrf_token: String,
    #[validate(custom = "validate::password")]
    pub password_new: Option<String>,
    #[validate(custom = "validate::password")]
    pub password_confirm: Option<String>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAuthPasswordUpdate {
    #[serde(default)]
    #[validate(custom = "validate::csrf_token")]
    pub csrf_token: String,
    #[serde(default)]
    #[validate(custom = "validate::password")]
    pub password: String,
    #[serde(default)]
    #[validate(custom = "validate::password")]
    pub password_new: String,
    #[serde(default)]
    #[validate(custom = "validate::password")]
    pub password_confirm: String,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAuthResponseQuery {
    #[serde(default)]
    #[validate(length(min = 1, max = 10))]
    pub response_type: String,
    #[serde(default)]
    #[validate(custom = "validate::code")]
    pub code: String,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAuthRegister {
    #[serde(default)]
    #[validate(custom = "validate::csrf_token")]
    pub csrf_token: String,
    #[serde(default)]
    #[validate(length(min = 1, max = 20))]
    pub register_type: String,
    #[validate(custom = "validate::name")]
    pub name: Option<String>,
    #[validate(custom = "validate::password")]
    pub password: Option<String>,
    #[validate(custom = "validate::password")]
    pub password_confirm: Option<String>,
    #[validate(length(min = 1, max = 5))]
    pub password_allow_reset: Option<String>,
    #[validate(custom = "validate::oauth2_provider")]
    pub oauth2_provider: Option<String>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestUserCreate {
    /// User name
    #[serde(default)]
    #[validate(custom = "validate::name")]
    pub name: String,
    /// User email address
    #[serde(default)]
    #[validate(email)]
    pub email: String,
    /// User locale
    #[serde(default)]
    #[validate(custom = "validate::locale")]
    pub locale: String,
    /// User timezone
    #[serde(default)]
    #[validate(custom = "validate::timezone")]
    pub timezone: String,
    /// User enable flag
    #[serde(default = "default_as_true")]
    pub enable: bool,
    /// User password
    #[validate]
    pub password: Option<RequestUserPasswordSet>,
    /// User access scope
    #[serde(default)]
    #[validate(custom = "validate::scope")]
    pub scope: String,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestUserPasswordSet {
    /// User password
    #[serde(default)]
    #[validate(custom = "validate::password")]
    pub password: String,
    /// User allow password reset flag
    #[serde(default)]
    pub allow_reset: bool,
    /// User require password update flag
    #[serde(default)]
    pub require_update: bool,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestUserRead {
    pub id: Option<Vec<Uuid>>,
    #[validate(custom = "validate::email_vec")]
    pub email: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestUserUpdate {
    pub id: Uuid,
    #[validate(custom = "validate::password")]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(custom = "validate::locale")]
    pub locale: Option<String>,
    #[validate(custom = "validate::timezone")]
    pub timezone: Option<String>,
    pub enable: Option<bool>,
    #[validate]
    pub password: Option<RequestUserPasswordUpdate>,
    #[validate]
    pub access: Option<RequestUserAccessUpdate>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestUserPasswordUpdate {
    pub allow_reset: Option<bool>,
    pub require_update: Option<bool>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestUserAccessUpdate {
    pub enable: Option<bool>,
    #[validate(custom = "validate::scope")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestUserDelete {
    pub id: Uuid,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestUserAccessRead {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseUser {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub locale: String,
    pub timezone: String,
    pub enable: bool,
    #[serde(rename = "static")]
    pub static_: bool,
    pub password: Option<ResponseUserPassword>,
    pub oauth2_provider: Vec<ResponseUserOauth2Provider>,
    pub oauth2_provider_count: i64,
    pub access: Option<ResponseAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseUserMany {
    pub data: Vec<ResponseUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseUserPassword {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub allow_reset: bool,
    pub require_update: bool,
    #[serde(rename = "static")]
    pub static_: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseUserOauth2Provider {
    pub created_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub oauth2_provider: String,
    pub sub: String,
    #[serde(rename = "static")]
    pub static_: bool,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestCsrf {
    #[serde(default)]
    #[validate(custom = "validate::csrf_token")]
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseCsrf {
    pub created_at: DateTime<Utc>,
    pub client_id: Uuid,
    pub token: String,
    pub ttl: DateTime<Utc>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAccessUpdate {
    pub user_id: Uuid,
    pub enable: bool,
    #[serde(default)]
    #[validate(custom = "validate::scope")]
    pub scope: String,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAccessRead {}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAccessDelete {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseAccess {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub client_id: Uuid,
    pub user_id: Uuid,
    pub enable: bool,
    pub scope: String,
    #[serde(rename = "static")]
    pub static_: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseAccessMany {
    pub data: Vec<ResponseAccess>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAuditCreate {
    /// Audit log user UUID
    pub user_id: Option<Uuid>,
    pub token_id: Option<Uuid>,
    pub api_key_id: Option<Uuid>,
    #[serde(default)]
    #[validate(custom = "validate::audit_type")]
    pub audit_type: String,
    /// Audit log subject
    #[serde(default)]
    #[validate(custom = "validate::audit_subject")]
    pub subject: Option<String>,
    /// Audit log data object
    pub data: Value,
    #[validate(range(min = 0))]
    pub status_code: Option<i16>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestSeekId {
    #[validate(custom = "validate::id")]
    pub id: Option<i64>,
    #[validate(range(min = 0))]
    pub limit: i64,
}

impl Default for RequestSeekId {
    fn default() -> Self {
        Self {
            id: None,
            limit: 10,
        }
    }
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestAuditRead {
    #[serde(default)]
    #[validate]
    pub seek: RequestSeekId,
    #[validate(custom = "validate::id_vec")]
    pub id: Option<Vec<i64>>,
    pub user_id: Option<Vec<Uuid>>,
    #[validate(custom = "validate::audit_type_vec")]
    pub audit_type: Option<Vec<String>>,
    #[validate(custom = "validate::audit_subject_vec")]
    pub subject: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseAudit {
    pub created_at: DateTime<Utc>,
    pub id: i64,
    pub client_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub token_id: Option<Uuid>,
    pub api_key_id: Option<Uuid>,
    pub audit_type: String,
    pub subject: Option<String>,
    pub status_code: Option<i16>,
    /// Audit log data object
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseAuditMany {
    pub data: Vec<ResponseAudit>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestApiKeyCreate {
    pub user_id: Uuid,
    #[serde(default)]
    #[validate(custom = "validate::name")]
    pub name: String,
    #[serde(default = "default_as_true")]
    pub enable: bool,
    #[serde(default)]
    #[validate(custom = "validate::scope")]
    pub scope: String,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestApiKeyRead {
    pub id: Option<Vec<Uuid>>,
    pub user_id: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestApiKeyUpdate {
    pub id: Uuid,
    #[validate(custom = "validate::name")]
    pub name: Option<String>,
    pub enable: Option<bool>,
}

#[derive(Debug, Default, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestApiKeyVerify {
    #[serde(default)]
    #[validate(custom = "validate::token")]
    pub key: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestApiKeyDelete {
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseApiKey {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub id: Uuid,
    pub client_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub enable: bool,
    pub scope: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseApiKeyMany {
    pub data: Vec<ResponseApiKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseClient {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub redirect_uri: String,
    pub enable: bool,
    pub scope: String,
    pub user_scope: String,
    pub register_enable: bool,
    pub register_scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct ResponseOpenidConfiguration {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub token_endpoint_auth_methods_supported: Vec<String>,
}

/// Client
#[derive(Debug, Clone)]
pub(crate) struct Client {
    pub server_authorize_uri: Url,
    pub server_token_uri: Url,
    pub server_introspect_uri: Url,
    pub client_id: Uuid,
    pub client_secret: String,
    pub redirect_uri: Url,
    pub client_name: String,
    pub client_uri: Url,
    pub enable: bool,
    pub scope: oauth2::Scope,
    pub user_scope: oauth2::Scope,
    pub register_enable: bool,
    pub register_scope: oauth2::Scope,
    pub ttl: ConfigOauth2ClientTtl,
    pub templates: ConfigOauth2ClientTemplates,
}

impl Client {
    pub fn client_name(&self) -> String {
        self.client_name.clone()
    }

    pub fn client_uri(&self) -> Url {
        self.client_uri.clone()
    }
}

impl oauth2::ClientIf for Client {
    fn server_authorize_uri(&self) -> Url {
        self.server_authorize_uri.clone()
    }
    fn server_token_uri(&self) -> Url {
        self.server_token_uri.clone()
    }
    fn client_id(&self) -> String {
        self.client_id.to_string()
    }
    fn client_secret(&self) -> String {
        self.client_secret.clone()
    }
    fn redirect_uri(&self) -> Url {
        self.redirect_uri.clone()
    }
}

impl oauth2::ResourceServerIf for Client {
    fn server_introspect_uri(&self) -> Url {
        self.server_introspect_uri.clone()
    }
    fn client_id(&self) -> String {
        self.client_id.to_string()
    }
    fn client_secret(&self) -> String {
        self.client_secret.clone()
    }
}

impl From<&Client> for ResponseClient {
    fn from(x: &Client) -> Self {
        Self {
            id: x.client_id.to_string(),
            name: x.client_name(),
            uri: x.client_uri().as_str().to_string(),
            redirect_uri: x.redirect_uri.as_str().to_string(),
            enable: x.enable,
            scope: x.scope.to_string(),
            user_scope: x.user_scope.to_string(),
            register_enable: x.register_enable,
            register_scope: x.register_scope.to_string(),
        }
    }
}
