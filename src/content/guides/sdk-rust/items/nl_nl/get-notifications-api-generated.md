## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| user_id | String | Nee |  |
| url_id | String | Nee |  |
| from_comment_id | String | Nee |  |
| viewed | bool | Nee |  |
| skip | f64 | Nee |  |

## Antwoord

Retourneert: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_notifications Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_notifications() -> Result<(), Error> {
    let params: GetNotificationsParams = GetNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-9a7b".to_string()),
        url_id: Some("news/article/launch-announcement".to_string()),
        from_comment_id: Some("cmt-1024".to_string()),
        viewed: Some(false),
        skip: Some(0.0),
    };
    let notifications: GetNotificationsResponse = get_notifications(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---