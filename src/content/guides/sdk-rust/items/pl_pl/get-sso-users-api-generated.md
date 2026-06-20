## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| skip | i32 | Nie |  |

## Odpowiedź

Zwraca: [`GetSsoUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_users_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_sso_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetSsoUsersResponse, Error> {
    let params: GetSsoUsersParams = GetSsoUsersParams {
        tenant_id: String::from("acme-corp-tenant"),
        skip: Some(10),
    };
    let sso_users: GetSsoUsersResponse = get_sso_users(&configuration, params).await?;
    Ok(sso_users)
}
[inline-code-end]

---