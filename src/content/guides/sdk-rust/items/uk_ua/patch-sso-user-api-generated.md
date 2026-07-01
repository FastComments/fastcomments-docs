## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_apisso_user_data | models::UpdateApissoUserData | Yes |  |
| update_comments | bool | No |  |

## Відповідь

Повертає: [`PatchSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_sso_user_api_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад patch_sso_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let update_data = models::UpdateApissoUserData {
        email: Some("john.doe@example.com".to_string()),
        name: Some("John Doe".to_string()),
    };
    let params = PatchSsoUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-12345".to_string(),
        update_apisso_user_data: update_data,
        update_comments: Some(true),
    };
    let _response: PatchSsoUserApiResponse = patch_sso_user(&config, params).await?;
    Ok(())
}
[inline-code-end]