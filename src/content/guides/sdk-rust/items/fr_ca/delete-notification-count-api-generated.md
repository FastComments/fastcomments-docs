## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |

## Réponse

Renvoie: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de delete_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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