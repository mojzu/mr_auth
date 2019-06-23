use manual::*;

// TODO(test): Refactor tests to this format.

#[test]
fn api_ping() {
    let client = create_client();
    ping_server(&client);
}

#[test]
fn api_user_create_ok() {
    let mut client = create_client();
    let (_service, service_key) = create_service_key(&client);
    let user_email = create_user_email();

    client.options.set_authorisation(&service_key.value);
    create_user(&client, true, "User Name", &user_email, None);
}

#[test]
fn api_user_create_bad_request_duplicate_user_email() {
    let mut client = create_client();
    let (_service, service_key) = create_service_key(&client);
    let user_email = create_user_email();

    client.options.set_authorisation(&service_key.value);
    create_user(&client, true, "User Name", &user_email, None);

    create_user_duplicate_email(&client, true, "User Name", &user_email, None);
}

// TODO(test): Create user bad requests.

#[test]
fn api_user_create_forbidden() {
    let mut client = create_client();
    let user_email = create_user_email();

    client.options.set_authorisation("invalid-service-key");
    create_user_forbidden(&client, true, "User Name", &user_email, None);
}

// #[test]
// fn api_user_list_email_eq() {
//     let mut client = create_client();
//     let (service, service_key) = create_service_key(&client);
//     let user_email = create_user_email();

//     client.options.set_authorisation(&service_key.value);
//     let user = create_user(&client, "User Name", &user_email, true, None);
// }

#[test]
fn guide_api_key() {
    let mut client = create_client();
    let (service, service_key) = create_service_key(&client);
    let user_email = create_user_email();

    client.options.set_authorisation(&service_key.value);
    let user = create_user(&client, true, "User Name", &user_email, None);
    let user_key = create_user_key(&client, "Key Name", &service.id, &user.id);

    verify_user_key(&client, &user_key);
    revoke_user_key(&client, &user_key.key);
    verify_user_key_bad_request(&client, &user_key.key);
}

#[test]
fn guide_login() {
    let mut client = create_client();
    let (service, service_key) = create_service_key(&client);
    let user_email = create_user_email();

    client.options.set_authorisation(&service_key.value);
    let user = create_user(&client, true, "User Name", &user_email, Some("guest"));
    let _user_key = create_user_key(&client, "Key Name", &service.id, &user.id);

    let user_token = local_login(&client, &user.id, &user_email, "guest");
    verify_user_token(&client, &user_token);
    let user_token = refresh_user_token(&client, &user_token);
    revoke_user_token(&client, &user_token.access_token);
    verify_user_token_bad_request(&client, &user_token.refresh_token);
}

#[test]
fn guide_oauth2_login() {
    let mut client = create_client();
    let (service, service_key) = create_service_key(&client);
    let user_email = create_user_email();

    client.options.set_authorisation(&service_key.value);
    let user = create_user(&client, true, "User Name", &user_email, Some("guest"));
    let _user_key = create_user_key(&client, "Key Name", &service.id, &user.id);

    microsoft_oauth2_request(&client);
}

#[test]
fn guide_reset_password() {
    let mut client = create_client();
    let (service, service_key) = create_service_key(&client);
    let user_email = create_user_email();

    client.options.set_authorisation(&service_key.value);
    let user = create_user(&client, true, "User Name", &user_email, Some("guest"));
    let _user_key = create_user_key(&client, "Key Name", &service.id, &user.id);

    local_password_reset(&client, &user_email);
}

// TODO(test): Update email/password guide tests.
