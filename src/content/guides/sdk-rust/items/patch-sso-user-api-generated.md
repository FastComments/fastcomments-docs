## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_apisso_user_data | models::UpdateApissoUserData | Yes |  |
| update_comments | bool | No |  |

## Response

Returns: [`PatchSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_sso_user_api_response.rs)

## Example

[inline-code-attrs-start title = 'patch_sso_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_patch_sso_user() -> Result<PatchSsoUserApiResponse, Error> {
    let params: PatchSsoUserParams = PatchSsoUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-1234".to_string(),
        update_apisso_user_data: models::UpdateApissoUserData {
            email: "jane.doe@acme.com".to_string(),
            display_name: "Jane Doe".to_string(),
            role: "editor".to_string(),
            external_id: "sso-5678".to_string(),
        },
        update_comments: Some(true),
    };
    let response: PatchSsoUserApiResponse = patch_sso_user(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
