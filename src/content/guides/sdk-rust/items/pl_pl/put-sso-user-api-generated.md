## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |
| update_apisso_user_data | models::UpdateApissoUserData | Tak |  |
| update_comments | bool | Nie |  |

## Odpowiedź

Zwraca: [`PutSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/put_sso_user_api_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład put_sso_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let update_data = UpdateApissoUserData {
        email: "jane.doe@example.com".to_string(),
        display_name: "Jane Doe".to_string(),
    };
    let params = PutSsoUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-12345".to_string(),
        update_apisso_user_data: update_data,
        update_comments: Some(true),
    };
    let _response = put_sso_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]