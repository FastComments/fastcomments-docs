## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| user_id | String | Non |  |
| url_id | String | Non |  |
| from_comment_id | String | Non |  |
| viewed | bool | Non |  |

## Réponse

Renvoie: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetNotificationCountParams = GetNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-67890".to_string()),
        url_id: Some("news/2026/03/25/election-updates".to_string()),
        from_comment_id: Some("cmt_42".to_string()),
        viewed: Some(false),
    };
    let response: GetNotificationCount200Response = get_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---