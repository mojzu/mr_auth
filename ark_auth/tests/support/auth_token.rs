#[macro_export]
macro_rules! auth_token_integration_test {
    () => {
        #[test]
        #[ignore]
        fn api_auth_token_verify_forbidden() {
            let client = client_create(Some(INVALID_SERVICE_KEY));
            let res = client.auth_token_verify(INVALID_UUID, None).unwrap_err();
            assert_eq!(res, Error::Request(RequestError::Forbidden));
        }

        #[test]
        #[ignore]
        fn api_auth_token_verify_bad_request_invalid_token() {
            let client = client_create(None);
            let (_service, service_key) = service_key_create(&client);

            let client = client_create(Some(&service_key.value));
            let res = client.auth_token_verify(INVALID_UUID, None).unwrap_err();
            assert_eq!(res, Error::Request(RequestError::BadRequest));
        }

        #[test]
        #[ignore]
        fn api_auth_token_verify_bad_request_unknown_user_key_for_service() {
            let client = client_create(None);
            let (service1, service1_key) = service_key_create(&client);
            let (_service2, service2_key) = service_key_create(&client);
            let user_email = email_create();

            let client = client_create(Some(&service1_key.value));
            let user = user_create(&client, true, USER_NAME, &user_email, Some(USER_PASSWORD));
            let _user_key = user_key_create(&client, KEY_NAME, &service1.id, &user.id);
            let user_token = auth_local_login(&client, &user.id, &user_email, USER_PASSWORD);

            let client = client_create(Some(&service2_key.value));
            let res = client
                .auth_token_verify(&user_token.access_token, None)
                .unwrap_err();
            assert_eq!(res, Error::Request(RequestError::BadRequest));
        }

        #[test]
        #[ignore]
        fn api_auth_token_verify_ok() {
            let client = client_create(None);
            let (service, service_key) = service_key_create(&client);
            let user_email = email_create();

            let client = client_create(Some(&service_key.value));
            let user = user_create(&client, true, USER_NAME, &user_email, Some(USER_PASSWORD));
            let _user_key = user_key_create(&client, KEY_NAME, &service.id, &user.id);
            let user_token = auth_local_login(&client, &user.id, &user_email, USER_PASSWORD);

            client
                .auth_token_verify(&user_token.access_token, None)
                .unwrap();
        }

        #[test]
        #[ignore]
        fn api_auth_token_refresh_forbidden() {
            let client = client_create(Some(INVALID_SERVICE_KEY));
            let res = client.auth_token_refresh(INVALID_UUID, None).unwrap_err();
            assert_eq!(res, Error::Request(RequestError::Forbidden));
        }

        #[test]
        #[ignore]
        fn api_auth_token_refresh_bad_request_invalid_token() {
            let client = client_create(None);
            let (_service, service_key) = service_key_create(&client);

            let client = client_create(Some(&service_key.value));
            let res = client.auth_token_refresh(INVALID_UUID, None).unwrap_err();
            assert_eq!(res, Error::Request(RequestError::BadRequest));
        }

        #[test]
        #[ignore]
        fn api_auth_token_refresh_bad_request_unknown_user_key_for_service() {
            let client = client_create(None);
            let (service1, service1_key) = service_key_create(&client);
            let (_service2, service2_key) = service_key_create(&client);
            let user_email = email_create();

            let client = client_create(Some(&service1_key.value));
            let user = user_create(&client, true, USER_NAME, &user_email, Some(USER_PASSWORD));
            let _user_key = user_key_create(&client, KEY_NAME, &service1.id, &user.id);
            let user_token = auth_local_login(&client, &user.id, &user_email, USER_PASSWORD);

            let client = client_create(Some(&service2_key.value));
            let res = client
                .auth_token_refresh(&user_token.refresh_token, None)
                .unwrap_err();
            assert_eq!(res, Error::Request(RequestError::BadRequest));
        }

        #[test]
        #[ignore]
        fn api_auth_token_refresh_bad_request_used_refresh_token() {
            let client = client_create(None);
            let (service, service_key) = service_key_create(&client);
            let user_email = email_create();

            let client = client_create(Some(&service_key.value));
            let user = user_create(&client, true, USER_NAME, &user_email, Some(USER_PASSWORD));
            let _user_key = user_key_create(&client, KEY_NAME, &service.id, &user.id);
            let user_token = auth_local_login(&client, &user.id, &user_email, USER_PASSWORD);

            user_token_verify(&client, &user_token);
            let user_token2 = user_token_refresh(&client, &user_token);
            client
                .auth_token_verify(&user_token2.access_token, None)
                .unwrap();

            let res = client
                .auth_token_refresh(&user_token.refresh_token, None)
                .unwrap_err();
            assert_eq!(res, Error::Request(RequestError::BadRequest));
        }

        #[test]
        #[ignore]
        fn api_auth_token_refresh_ok() {
            let client = client_create(None);
            let (service, service_key) = service_key_create(&client);
            let user_email = email_create();

            let client = client_create(Some(&service_key.value));
            let user = user_create(&client, true, USER_NAME, &user_email, Some(USER_PASSWORD));
            let _user_key = user_key_create(&client, KEY_NAME, &service.id, &user.id);
            let user_token = auth_local_login(&client, &user.id, &user_email, USER_PASSWORD);

            user_token_verify(&client, &user_token);
            let user_token = user_token_refresh(&client, &user_token);
            client
                .auth_token_verify(&user_token.access_token, None)
                .unwrap();
        }

        #[test]
        #[ignore]
        fn api_auth_token_revoke_forbidden() {
            let client = client_create(Some(INVALID_SERVICE_KEY));
            let res = client.auth_token_revoke(INVALID_UUID, None).unwrap_err();
            assert_eq!(res, Error::Request(RequestError::Forbidden));
        }

        #[test]
        #[ignore]
        fn api_auth_token_revoke_bad_request_invalid_token() {
            let client = client_create(None);
            let (_service, service_key) = service_key_create(&client);

            let client = client_create(Some(&service_key.value));
            let res = client.auth_token_revoke(INVALID_UUID, None).unwrap_err();
            assert_eq!(res, Error::Request(RequestError::BadRequest));
        }

        #[test]
        #[ignore]
        fn api_auth_token_revoke_bad_request_unknown_user_key_for_service() {
            let client = client_create(None);
            let (service1, service1_key) = service_key_create(&client);
            let (_service2, service2_key) = service_key_create(&client);
            let user_email = email_create();

            let client = client_create(Some(&service1_key.value));
            let user = user_create(&client, true, USER_NAME, &user_email, Some(USER_PASSWORD));
            let _user_key = user_key_create(&client, KEY_NAME, &service1.id, &user.id);
            let user_token = auth_local_login(&client, &user.id, &user_email, USER_PASSWORD);

            let client = client_create(Some(&service2_key.value));
            let res = client
                .auth_token_revoke(&user_token.refresh_token, None)
                .unwrap_err();
            assert_eq!(res, Error::Request(RequestError::BadRequest));
        }

        #[test]
        #[ignore]
        fn api_auth_token_revoke_ok() {
            let client = client_create(None);
            let (service, service_key) = service_key_create(&client);
            let user_email = email_create();

            let client = client_create(Some(&service_key.value));
            let user = user_create(&client, true, USER_NAME, &user_email, Some(USER_PASSWORD));
            let _user_key = user_key_create(&client, KEY_NAME, &service.id, &user.id);

            let user_token = auth_local_login(&client, &user.id, &user_email, USER_PASSWORD);

            user_token_verify(&client, &user_token);
            client
                .auth_token_revoke(&user_token.access_token, None)
                .unwrap();
            let res = client
                .auth_token_verify(&user_token.access_token, None)
                .unwrap_err();
            assert_eq!(res, Error::Request(RequestError::BadRequest));
        }
    };
}
