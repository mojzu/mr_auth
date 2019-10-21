use crate::{impl_enum_to_from_string, Driver, KeyWithValue, Service, User};
use chrono::{DateTime, Utc};
use serde::ser::Serialize;
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

/// Audit type maximum length.
pub const AUDIT_TYPE_MAX_LEN: usize = 200;

/// Audit subject maximum length.
pub const AUDIT_SUBJECT_MAX_LEN: usize = 200;

/// Audit types.
#[derive(Debug, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub enum AuditType {
    Metrics,
    AuditList,
    AuditCreate,
    AuditRead,
    AuditUpdate,
    KeyList,
    KeyCreate,
    KeyRead,
    KeyUpdate,
    KeyDelete,
    ServiceList,
    ServiceCreate,
    ServiceRead,
    ServiceUpdate,
    ServiceDelete,
    UserList,
    UserCreate,
    UserRead,
    UserUpdate,
    UserDelete,
    AuthLocalLogin,
    AuthLocalResetPassword,
    AuthLocalResetPasswordConfirm,
    AuthLocalUpdateEmail,
    AuthLocalUpdateEmailRevoke,
    AuthLocalUpdatePassword,
    AuthLocalUpdatePasswordRevoke,
    AuthGithubOauth2Url,
    AuthGithubOauth2Callback,
    AuthMicrosoftOauth2Url,
    AuthMicrosoftOauth2Callback,
    AuthOauth2Login,
    AuthKeyVerify,
    AuthKeyRevoke,
    AuthTokenVerify,
    AuthTokenRefresh,
    AuthTokenRevoke,
    AuthTotp,
    AuthCsrfCreate,
    AuthCsrfVerify,
}

impl_enum_to_from_string!(AuditType, "Sso");

/// Audit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audit {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub id: Uuid,
    pub user_agent: String,
    pub remote: String,
    pub forwarded: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub subject: Option<String>,
    pub data: Value,
    pub key_id: Option<Uuid>,
    pub service_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub user_key_id: Option<Uuid>,
}

impl fmt::Display for Audit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Audit {}", self.id)?;
        write!(f, "\n\tcreated_at {}", self.created_at)?;
        write!(f, "\n\tuser_agent {}", self.user_agent)?;
        write!(f, "\n\tremote {}", self.remote)?;
        if let Some(forwarded) = &self.forwarded {
            write!(f, "\n\tforwarded {}", forwarded)?;
        }
        write!(f, "\n\ttype {}", self.type_)?;
        if let Some(subject) = &self.subject {
            write!(f, "\n\tsubject {}", subject)?;
        }
        write!(f, "\n\tdata {}", self.data)?;
        if let Some(key_id) = &self.key_id {
            write!(f, "\n\tkey_id {}", key_id)?;
        }
        if let Some(service_id) = &self.service_id {
            write!(f, "\n\tservice_id {}", service_id)?;
        }
        if let Some(user_id) = &self.user_id {
            write!(f, "\n\tuser_id {}", user_id)?;
        }
        if let Some(user_key_id) = &self.user_key_id {
            write!(f, "\n\tuser_key_id {}", user_key_id)?;
        }
        Ok(())
    }
}

/// Audit create.
#[derive(Debug)]
pub struct AuditCreate {
    pub meta: AuditMeta,
    pub type_: String,
    pub subject: Option<String>,
    pub data: Option<Value>,
    pub key_id: Option<Uuid>,
    pub service_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub user_key_id: Option<Uuid>,
}

impl AuditCreate {
    pub fn new(
        meta: AuditMeta,
        type_: String,
        subject: Option<String>,
        data: Option<Value>,
    ) -> Self {
        Self {
            meta,
            type_,
            subject,
            data,
            key_id: None,
            service_id: None,
            user_id: None,
            user_key_id: None,
        }
    }

    pub fn key_id(mut self, key_id: Option<Uuid>) -> Self {
        self.key_id = key_id;
        self
    }

    pub fn service_id(mut self, service_id: Option<Uuid>) -> Self {
        self.service_id = service_id;
        self
    }

    pub fn user_id(mut self, user_id: Option<Uuid>) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn user_key_id(mut self, user_key_id: Option<Uuid>) -> Self {
        self.user_key_id = user_key_id;
        self
    }
}

/// Audit create 2.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditCreate2 {
    #[serde(rename = "type")]
    type_: String,
    subject: Option<String>,
    data: Option<Value>,
}

impl AuditCreate2 {
    pub fn new(type_: String, subject: Option<String>, data: Option<Value>) -> Self {
        Self {
            type_,
            subject,
            data,
        }
    }
}

/// Audit list query.
#[derive(Debug)]
pub enum AuditListQuery {
    /// Where created less than or equal.
    CreatedLe(DateTime<Utc>, i64, Option<Uuid>),
    /// Where created greater than or equal.
    CreatedGe(DateTime<Utc>, i64, Option<Uuid>),
    /// Where created less than or equal and greater than or equal.
    CreatedLeAndGe(DateTime<Utc>, DateTime<Utc>, i64, Option<Uuid>),
}

/// Audit list filter.
#[derive(Debug)]
pub struct AuditListFilter {
    pub id: Option<Vec<Uuid>>,
    pub type_: Option<Vec<String>>,
    pub subject: Option<Vec<String>>,
    pub service_id: Option<Vec<Uuid>>,
    pub user_id: Option<Vec<Uuid>>,
}

/// Audit list.
#[derive(Debug)]
pub struct AuditList<'a> {
    pub query: &'a AuditListQuery,
    pub filter: &'a AuditListFilter,
    pub service_id_mask: Option<Uuid>,
}

/// Audit read.
#[derive(Debug)]
pub struct AuditRead {
    pub id: Uuid,
    pub service_id_mask: Option<Uuid>,
}

impl AuditRead {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            service_id_mask: None,
        }
    }

    pub fn service_id_mask(mut self, service_id_mask: Option<Uuid>) -> Self {
        self.service_id_mask = service_id_mask;
        self
    }
}

/// Audit update.
#[derive(Debug)]
pub struct AuditUpdate {
    pub subject: Option<String>,
    pub data: Option<Value>,
}

/// Audit metadata.
///
/// HTTP request information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditMeta {
    user_agent: String,
    remote: String,
    forwarded: Option<String>,
}

impl AuditMeta {
    /// Create audit metadata from parameters.
    pub fn new<T1: Into<String>, T2: Into<Option<String>>>(
        user_agent: T1,
        remote: T1,
        forwarded: T2,
    ) -> Self {
        AuditMeta {
            user_agent: user_agent.into(),
            remote: remote.into(),
            forwarded: forwarded.into(),
        }
    }

    /// User agent string reference.
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Remote IP string reference.
    pub fn remote(&self) -> &str {
        &self.remote
    }

    /// Forwarded for header optional string reference.
    pub fn forwarded(&self) -> Option<&str> {
        self.forwarded.as_ref().map(|x| &**x)
    }
}

/// Audit log builder pattern.
#[derive(Debug)]
pub struct AuditBuilder {
    meta: AuditMeta,
    type_: AuditType,
    key: Option<Uuid>,
    service: Option<Uuid>,
    user: Option<Uuid>,
    user_key: Option<Uuid>,
}

impl AuditBuilder {
    /// Create a new audit log builder with required parameters.
    pub fn new(meta: AuditMeta, type_: AuditType) -> Self {
        AuditBuilder {
            meta,
            type_,
            key: None,
            service: None,
            user: None,
            user_key: None,
        }
    }

    pub fn key(&mut self, key: Option<&KeyWithValue>) -> &mut Self {
        self.key = key.map(|x| x.id);
        self
    }

    pub fn service(&mut self, service: Option<&Service>) -> &mut Self {
        self.service = service.map(|x| x.id);
        self
    }

    pub fn user(&mut self, user: Option<&User>) -> &mut Self {
        self.user = user.map(|x| x.id);
        self
    }

    pub fn user_id(&mut self, user: Option<Uuid>) -> &mut Self {
        self.user = user;
        self
    }

    pub fn user_key(&mut self, key: Option<&KeyWithValue>) -> &mut Self {
        self.user_key = key.map(|x| x.id);
        self
    }

    pub fn user_key_id(&mut self, key: Option<Uuid>) -> &mut Self {
        self.user_key = key;
        self
    }

    /// Get reference to metadata.
    pub fn meta(&self) -> &AuditMeta {
        &self.meta
    }

    /// Create audit log from parameters.
    pub fn create(&self, driver: &dyn Driver, create: AuditCreate2) -> DriverResult<Audit> {
        let data = AuditCreate::new(self.meta.clone(), create.type_, create.subject, create.data)
            .key_id(self.key)
            .service_id(self.service)
            .user_id(self.user)
            .user_key_id(self.user_key);
        driver.audit_create(&data)
    }

    /// Create audit log with data.
    pub fn create_data<S: Serialize>(
        &self,
        driver: &dyn Driver,
        subject: Option<String>,
        data: Option<S>,
    ) -> DriverResult<Audit> {
        let data = data.map(|x| serde_json::to_value(x).unwrap());
        let audit_data = AuditCreate::new(
            self.meta.clone(),
            self.type_.to_string().unwrap(),
            subject,
            data,
        )
        .key_id(self.key)
        .service_id(self.service)
        .user_id(self.user)
        .user_key_id(self.user_key);
        driver.audit_create(&audit_data)
    }
}

/// Audit subject trait.
pub trait AuditSubject {
    /// Return subject value for audit log.
    fn subject(&self) -> String;
}

/// Audit diff trait.
pub trait AuditDiff {
    /// Return diff object for audit log.
    fn diff(&self, other: &Self) -> Value;
}

/// Audit diff builder pattern.
///
/// Internal structure is:
/// key -> previous value -> current value.
#[derive(Debug)]
pub struct AuditDiffBuilder {
    data: Vec<(String, String, String)>,
}

impl Default for AuditDiffBuilder {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl AuditDiffBuilder {
    /// Compare 2 versions of a value, if different push a row to diff data.
    pub fn compare<T: PartialEq + fmt::Display>(
        mut self,
        key: &str,
        current: &T,
        previous: &T,
    ) -> Self {
        if current != previous {
            self.data.push((
                String::from(key),
                format!("{}", previous),
                format!("{}", current),
            ))
        }
        self
    }

    /// Serialise diff data into Value.
    pub fn into_value(self) -> Value {
        Self::typed_data("diff", self.data)
    }

    /// Wrap serialisable data in object with type property.
    pub fn typed_data<T: Into<String>, D1: Serialize>(type_: T, data: D1) -> Value {
        #[derive(Serialize)]
        struct TypedData<D2: Serialize> {
            #[serde(rename = "type")]
            type_: String,
            data: D2,
        }
        let v = TypedData {
            type_: type_.into(),
            data,
        };
        serde_json::to_value(v).unwrap()
    }
}