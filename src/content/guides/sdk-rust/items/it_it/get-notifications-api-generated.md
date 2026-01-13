## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| user_id | String | No |  |
| url_id | String | No |  |
| from_comment_id | String | No |  |
| viewed | bool | No |  |
| skip | f64 | No |  |

## Risposta

Restituisce: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notifications() -> Result<(), Error> {
    let params: GetNotificationsParams = GetNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-1234".to_string()),
        url_id: Some("news/politics/article-2026-01-12".to_string()),
        from_comment_id: Some("cmt-98765".to_string()),
        viewed: Some(false),
        skip: Some(0.0),
    };
    let notifications: GetNotifications200Response = get_notifications(&configuration, params).await?;
    let _ = notifications;
    Ok(())
}
[inline-code-end]

---