## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| email | String | Tak |  |

## Odpowiedź

Zwraca: [`GetSsoUserByEmailApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_user_by_email_api_response.rs)

## Przykład

[inline-code-attrs-start title = 'get_sso_user_by_email Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let tenant_id = Some("acme-corp".to_string());
    let email = Some("john.doe@example.com".to_string());
    let params = GetSsoUserByEmailParams {
        tenant_id: tenant_id.unwrap(),
        email: email.unwrap(),
    };
    let _response = get_sso_user_by_email(configuration, params).await?;
    Ok(())
}
[inline-code-end]