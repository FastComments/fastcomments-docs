## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| user_id | String | Nein |  |
| url_id | String | Nein |  |
| from_comment_id | String | Nein |  |
| viewed | bool | Nein |  |
| skip | f64 | Nein |  |

## Antwort

Rückgabe: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_notifications Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notifications(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-123".to_string()),
        url_id: Some("news/article".to_string()),
        from_comment_id: Some("cmt-456".to_string()),
        viewed: Some(true),
        skip: Some(0.0),
    };
    let _response = get_notifications(configuration, params).await?;
    Ok(())
}
[inline-code-end]