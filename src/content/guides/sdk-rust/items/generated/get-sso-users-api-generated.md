## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| skip | i32 | No |  |

## Response

Returns: [`GetSsoUsers200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_users_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_sso_users Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_sso_users() -> Result<(), Error> {
    let params: GetSsoUsersParams = GetSsoUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10),
    };
    let _users_response: GetSsoUsers200Response = get_sso_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
