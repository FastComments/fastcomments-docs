## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |
| update_notification_body | models::UpdateNotificationBody | Tak |  |
| user_id | String | Nie |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'update_notification Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateNotificationParams = UpdateNotificationParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "notif-12345".to_string(),
    update_notification_body: models::UpdateNotificationBody {
        name: "Article Comments Webhook".to_string(),
        url: "https://hooks.acme-corp.com/article-comments".to_string(),
        enabled: true,
        events: vec!["comment.created".to_string(), "comment.flagged".to_string()],
    },
    user_id: Some("moderator-42".to_string()),
};

let response: FlagCommentPublic200Response = update_notification(&configuration, params).await?;
[inline-code-end]

---