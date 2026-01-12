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
async fn run() -> Result<(), Error> {
    let params: PatchSsoUserParams = PatchSsoUserParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("user-1234"),
        update_apisso_user_data: models::UpdateApissoUserData {
            email: String::from("jane.doe@acme.com"),
            display_name: Some(String::from("Jane Doe")),
            external_id: Some(String::from("sso-9876")),
            roles: Some(vec![String::from("editor")]),
            active: Some(true),
        },
        update_comments: Some(true),
    };

    let patched_user: PatchSsoUserApiResponse = patch_sso_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
