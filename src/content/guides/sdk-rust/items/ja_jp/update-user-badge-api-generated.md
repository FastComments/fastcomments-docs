## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| update_user_badge_params | models::UpdateUserBadgeParams | はい |  |

## レスポンス

戻り値: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_badge_200_response.rs)

## 例

[inline-code-attrs-start title = 'update_user_badge の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateUserBadgeParams = UpdateUserBadgeParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "badge-verified-001".to_string(),
    update_user_badge_params: models::UpdateUserBadgeParams {
        name: Some("Verified Contributor".to_string()),
        description: Some("Awarded for consistent, high-quality contributions".to_string()),
        icon_url: Some("https://assets.acme.com/badges/verified.png".to_string()),
        color_hex: Some("#1E90FF".to_string()),
        min_posts: Some(100u32),
        active: Some(true),
    },
};
let response: UpdateUserBadge200Response = update_user_badge(&configuration, params).await?;
[inline-code-end]

---