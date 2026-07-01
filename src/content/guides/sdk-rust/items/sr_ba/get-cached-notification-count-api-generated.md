## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Odgovor

Vraća: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_cached_notification_count_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer get_cached_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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