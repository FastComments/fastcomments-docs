---
## Параметри

| Име | Тип | Захтевано | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| user_id | String | Не |  |
| url_id | String | Не |  |
| from_comment_id | String | Не |  |
| viewed | bool | Не |  |

## Одговор

Враћа: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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