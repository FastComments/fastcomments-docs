## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Отговор

Връща: [`ApiGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_response.rs)

## Пример

[inline-code-attrs-start title = 'get_user_badge Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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