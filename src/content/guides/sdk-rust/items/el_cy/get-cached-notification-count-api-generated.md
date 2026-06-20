## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| id | String | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_cached_notification_count_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_cached_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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