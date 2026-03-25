## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| id | String | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_cached_notification_count_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_cached_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
pub async fn example_get_cached_notification_count(configuration: &configuration::Configuration) -> Result<GetCachedNotificationCount200Response, Error> {
    let params: GetCachedNotificationCountParams = GetCachedNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-12345".to_string(),
    };
    let response: GetCachedNotificationCount200Response = get_cached_notification_count(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---