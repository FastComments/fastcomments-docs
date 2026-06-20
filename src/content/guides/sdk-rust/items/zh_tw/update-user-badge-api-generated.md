## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| update_user_badge_params | models::UpdateUserBadgeParams | 是 |  |

## 回應

回傳: [`ApiEmptySuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_success_response.rs)

## 範例

[inline-code-attrs-start title = 'update_user_badge 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update_badge() -> Result<ApiEmptySuccessResponse, Error> {
    let params: UpdateUserBadgeParams = UpdateUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-8742".to_string(),
        update_user_badge_params: models::UpdateUserBadgeParams {
            name: Some("Top Contributor".to_string()),
            description: Some("Awarded for 100 helpful comments".to_string()),
            icon_url: Some("https://assets.acme.com/badges/top-contributor.png".to_string()),
            expires_at: None,
            is_visible: Some(true),
        },
    };
    let response: ApiEmptySuccessResponse = update_user_badge(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---