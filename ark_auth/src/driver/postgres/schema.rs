table! {
    /// Representation of the `auth_audit` table.
    ///
    /// (Automatically generated by Diesel.)
    auth_audit (audit_id) {
        /// The `created_at` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `audit_id` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        audit_id -> Uuid,
        /// The `audit_user_agent` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        audit_user_agent -> Varchar,
        /// The `audit_remote` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        audit_remote -> Varchar,
        /// The `audit_forwarded` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        audit_forwarded -> Nullable<Varchar>,
        /// The `audit_path` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        audit_path -> Varchar,
        /// The `audit_data` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Jsonb`.
        ///
        /// (Automatically generated by Diesel.)
        audit_data -> Jsonb,
        /// The `key_id` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        key_id -> Nullable<Uuid>,
        /// The `service_id` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Nullable<Uuid>,
        /// The `user_id` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Nullable<Uuid>,
        /// The `user_key_id` column of the `auth_audit` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        user_key_id -> Nullable<Uuid>,
    }
}

table! {
    /// Representation of the `auth_csrf` table.
    ///
    /// (Automatically generated by Diesel.)
    auth_csrf (csrf_key) {
        /// The `created_at` column of the `auth_csrf` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `csrf_key` column of the `auth_csrf` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        csrf_key -> Varchar,
        /// The `csrf_value` column of the `auth_csrf` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        csrf_value -> Varchar,
        /// The `csrf_ttl` column of the `auth_csrf` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        csrf_ttl -> Timestamptz,
        /// The `service_id` column of the `auth_csrf` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Uuid,
    }
}

table! {
    /// Representation of the `auth_key` table.
    ///
    /// (Automatically generated by Diesel.)
    auth_key (key_id) {
        /// The `created_at` column of the `auth_key` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `auth_key` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
        /// The `key_id` column of the `auth_key` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        key_id -> Uuid,
        /// The `key_is_enabled` column of the `auth_key` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        key_is_enabled -> Bool,
        /// The `key_is_revoked` column of the `auth_key` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        key_is_revoked -> Bool,
        /// The `key_name` column of the `auth_key` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        key_name -> Varchar,
        /// The `key_value` column of the `auth_key` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        key_value -> Varchar,
        /// The `service_id` column of the `auth_key` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Nullable<Uuid>,
        /// The `user_id` column of the `auth_key` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Nullable<Uuid>,
    }
}

table! {
    /// Representation of the `auth_service` table.
    ///
    /// (Automatically generated by Diesel.)
    auth_service (service_id) {
        /// The `created_at` column of the `auth_service` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `auth_service` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
        /// The `service_id` column of the `auth_service` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Uuid,
        /// The `service_is_enabled` column of the `auth_service` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        service_is_enabled -> Bool,
        /// The `service_name` column of the `auth_service` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        service_name -> Varchar,
        /// The `service_url` column of the `auth_service` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        service_url -> Varchar,
    }
}

table! {
    /// Representation of the `auth_user` table.
    ///
    /// (Automatically generated by Diesel.)
    auth_user (user_id) {
        /// The `created_at` column of the `auth_user` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `auth_user` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
        /// The `user_id` column of the `auth_user` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Uuid,
        /// The `user_is_enabled` column of the `auth_user` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        user_is_enabled -> Bool,
        /// The `user_name` column of the `auth_user` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        user_name -> Varchar,
        /// The `user_email` column of the `auth_user` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        user_email -> Varchar,
        /// The `user_password_hash` column of the `auth_user` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        user_password_hash -> Nullable<Varchar>,
    }
}

joinable!(auth_audit -> auth_service (service_id));
joinable!(auth_audit -> auth_user (user_id));
joinable!(auth_csrf -> auth_service (service_id));
joinable!(auth_key -> auth_service (service_id));
joinable!(auth_key -> auth_user (user_id));

allow_tables_to_appear_in_same_query!(auth_audit, auth_csrf, auth_key, auth_service, auth_user,);
