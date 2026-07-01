## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja | |
| id | String | Ja | |

## Antwort

Rückgabe: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_cached_notification_count_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_cached_notification_count Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notification_count() -> Result<(), Error> {
    let params = GetCachedNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
    };
    let response = get_cached_notification_count(&configuration, params).await?;
    let _ = response.user_notification_count;
    Ok(())
}
[inline-code-end]