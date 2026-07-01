## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| user_id | String | Non |  |
| url_id | String | Non |  |
| from_comment_id | String | Non |  |
| viewed | bool | Non |  |
| skip | f64 | Non |  |

## Réponse

Retourne : [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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