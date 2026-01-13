## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenant_id | String | Oui |  |
| user_id | String | Non |  |
| url_id | String | Non |  |
| from_comment_id | String | Non |  |
| viewed | bool | Non |  |

## Réponse

Retourne : [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_notification_count() -> Result<GetNotificationCount200Response, Error> {
    let params: GetNotificationCountParams = GetNotificationCountParams {
        tenant_id: String::from("acme-corp-tenant"),
        user_id: Some(String::from("user-12345")),
        url_id: Some(String::from("news/article/2026/product-launch")),
        from_comment_id: Some(String::from("cmt-000987")),
        viewed: Some(false),
    };
    let response: GetNotificationCount200Response = get_notification_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---