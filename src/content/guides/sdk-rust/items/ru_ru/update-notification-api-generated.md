---
## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| update_notification_body | models::UpdateNotificationBody | Да |  |
| user_id | String | Нет |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример update_notification'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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