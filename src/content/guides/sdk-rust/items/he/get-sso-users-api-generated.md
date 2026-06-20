## פרמטרים

| שם | Type | Required | תיאור |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| skip | i32 | No |  |

## תגובה

מחזיר: [`GetSsoUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_users_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_sso_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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