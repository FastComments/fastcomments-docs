## Parameters

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|----------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |
| update_user_badge_params | models::UpdateUserBadgeParams | Evet |  |

## Response

Döndürür: [`ApiEmptySuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_success_response.rs)

## Örnek

[inline-code-attrs-start title = 'update_user_badge Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = UpdateUserBadgeParams {
        tenant_id: "acme-corp".to_string(),
        id: "user-42".to_string(),
        update_user_badge_params: models::UpdateUserBadgeParams {
            badge_name: "contributor".to_string(),
            expires_at: Some("2025-12-31T23:59:59Z".to_string()),
        },
    };
    let _resp = update_user_badge(configuration, params).await?;
    Ok(())
}
[inline-code-end]