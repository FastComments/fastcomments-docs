---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |

## Yanıt

Dönüş: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Örnek

[inline-code-attrs-start title = 'delete_notification_count Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn delete_notification_count_example() -> Result<(), Error> {
    let params = DeleteNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/notification-827b".to_string(),
        if_match: Some("W/\"etag-827b\"".to_string()),
    };
    let _response: ApiEmptyResponse = delete_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---