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
async fn update_user_example() -> Result<(), Error> {
    let params: PatchSsoUserParams = PatchSsoUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "jane.doe@acme.com".to_string(),
        update_apisso_user_data: models::UpdateApissoUserData {
            email: Some("jane.doe@acme.com".to_string()),
            first_name: Some("Jane".to_string()),
            last_name: Some("Doe".to_string()),
            role: Some("editor".to_string()),
            active: Some(true),
        },
        update_comments: Some(true),
    };
    let response: PatchSsoUserApiResponse = patch_sso_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
