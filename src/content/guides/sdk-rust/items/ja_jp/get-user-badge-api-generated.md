## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |

## レスポンス

返り値: [`ApiGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_response.rs)

## 例

[inline-code-attrs-start title = 'get_user_badge の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_user_badge() -> Result<ApiGetUserBadgeResponse, Error> {
    let params: GetUserBadgeParams = GetUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-moderator".to_string(),
        include_inactive: Some(false),
    };
    let badge: ApiGetUserBadgeResponse = get_user_badge(&configuration, params).await?;
    Ok(badge)
}
[inline-code-end]

---