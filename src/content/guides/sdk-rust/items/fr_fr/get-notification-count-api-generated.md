## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| user_id | String | Non |  |
| url_id | String | Non |  |
| from_comment_id | String | Non |  |
| viewed | bool | Non |  |

## Réponse

Renvoie: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetNotificationCountParams = GetNotificationCountParams {
    tenant_id: "acme-corp-tenant".to_string(),
    user_id: Some("user-123".to_string()),
    url_id: Some("news/article/2026/06/19".to_string()),
    from_comment_id: Some("cmt-98765".to_string()),
    viewed: Some(false),
};
let notification_count: GetNotificationCountResponse = get_notification_count(configuration, params).await?;
[inline-code-end]

---