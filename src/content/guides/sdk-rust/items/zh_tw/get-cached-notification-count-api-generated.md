## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 回應

回傳: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_cached_notification_count_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_cached_notification_count 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_cached_notification_count() -> Result<(), Error> {
    let params: GetCachedNotificationCountParams = GetCachedNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-12345".to_string(),
    };
    let preferred_channel: Option<String> = Some("email".to_string());
    let response: GetCachedNotificationCount200Response =
        get_cached_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---