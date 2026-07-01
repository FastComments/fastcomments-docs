## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |
| delete_comments | bool | Nie |  |
| comment_delete_mode | String | Nie |  |

## Response

Zwraca: [`DeleteSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_sso_user_api_response.rs)

## Przykład

[inline-code-attrs-start title = 'delete_sso_user Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteSsoUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-42".to_string(),
        delete_comments: Some(true),
        comment_delete_mode: Some("soft".to_string()),
    };
    let _response: DeleteSsoUserApiResponse = delete_sso_user(&config, params).await?;
    Ok(())
}
[inline-code-end]