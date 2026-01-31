## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_apisso_user_data | models::UpdateApissoUserData | Yes |  |
| update_comments | bool | No |  |

## Response

Returns: [`PutSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/put_sso_user_api_response.rs)

## Example

[inline-code-attrs-start title = 'put_sso_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: PutSsoUserParams = PutSsoUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "john.doe@acme.com".to_string(),
        update_apisso_user_data: models::UpdateApissoUserData {
            display_name: "John Doe".to_string(),
            email: "john.doe@acme.com".to_string(),
            role: "editor".to_string(),
        },
        update_comments: Some(true),
    };

    let response: PutSsoUserApiResponse = put_sso_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
