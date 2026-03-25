## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 回應

回傳：[`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_badge_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_user_badge 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_user_badge() -> Result<GetUserBadge200Response, Error> {
    let params = GetUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "verified-journalist-badge-001".to_string(),
        locale: Some("en-US".to_string()),
    };
    let badge: GetUserBadge200Response = get_user_badge(&configuration, params).await?;
    Ok(badge)
}
[inline-code-end]

---