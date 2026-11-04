Включение или отключение уведомлений для страницы. Когда пользователи подписаны на страницу, уведомления создаются
для новых корневых комментариев, а также

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| url | String | Да |  |
| page_title | String | Да |  |
| subscribed_or_unsubscribed | String | Да |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример update_user_notification_page_subscription_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<UpdateUserNotificationPageSubscriptionStatusResponse, Error> {
    let params = UpdateUserNotificationPageSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        url_id: "news-article-2024".to_owned(),
        url: "https://news.example.com/articles/rust".to_owned(),
        page_title: "Rust Dominates the Programming World".to_owned(),
        subscribed_or_unsubscribed: "subscribed".to_owned(),
        sso: Some("sso-token-abc".to_owned()),
    };
    update_user_notification_page_subscription_status(&configuration, params).await
}
[inline-code-end]