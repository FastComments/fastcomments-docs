## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| email | String | Yes |  |

## Response

Returns: [`GetSsoUserByEmailApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_user_by_email_api_response.rs)

## Example

[inline-code-attrs-start title = 'get_sso_user_by_email Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn get_user_by_email_example() -> Result<GetSsoUserByEmailApiResponse, Error> {
    let params: GetSsoUserByEmailParams = GetSsoUserByEmailParams {
        tenant_id: "acme-corp-tenant".to_string(),
        email: "jane.doe@acme.com".to_string(),
        include_profile: Some(true),
    };
    let user: GetSsoUserByEmailApiResponse = get_sso_user_by_email(&configuration, params).await?;
    Ok(user)
}
[inline-code-end]
