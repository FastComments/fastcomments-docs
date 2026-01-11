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
async fn fetch_sso_user(configuration: &configuration::Configuration) -> Result<GetSsoUserByIdApiResponse, Error> {
    let params: GetSsoUserByIdParams = GetSsoUserByIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "sso-user-98765".to_string(),
    };
    let user: GetSsoUserByIdApiResponse = get_sso_user_by_id(configuration, params).await?;
    Ok(user)
}
[inline-code-end]
