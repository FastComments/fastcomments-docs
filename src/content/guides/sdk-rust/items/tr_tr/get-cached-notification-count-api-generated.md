## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |

## Yanıt

Döndürür: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_cached_notification_count_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_cached_notification_count Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_cached_notification_count() -> Result<GetCachedNotificationCountResponse, Error> {
    let params: GetCachedNotificationCountParams = GetCachedNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/12345".to_string(),
    };
    let response: GetCachedNotificationCountResponse = get_cached_notification_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---