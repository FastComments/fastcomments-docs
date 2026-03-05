## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetSsoUserByIdApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_user_by_id_api_response.rs)

## Example

[inline-code-attrs-start title = 'get_sso_user_by_id Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_sso_user() -> Result<GetSsoUserByIdApiResponse, Error> {
    let params = GetSsoUserByIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "jdoe-89".to_string(),
        expand: Some(vec!["profile".to_string(), "roles".to_string()]),
    };
    let sso_user: GetSsoUserByIdApiResponse = get_sso_user_by_id(&configuration, params).await?;
    Ok(sso_user)
}
[inline-code-end]
