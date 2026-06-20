Увімкнення або вимкнення сповіщень для сторінки. Коли користувачі підписані на сторінку, створюються сповіщення про нові кореневі коментарі, а також

## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| url | String | Так |  |
| page_title | String | Так |  |
| subscribed_or_unsubscribed | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## Приклад

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<UpdateUserNotificationPageSubscriptionStatusResponse, Error> {
    let params: UpdateUserNotificationPageSubscriptionStatusParams = UpdateUserNotificationPageSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/rocket-launch-2026".to_string(),
        url: "https://acme.example.com/news/rocket-launch-2026".to_string(),
        page_title: "Acme Rocket Launch — June 2026".to_string(),
        subscribed_or_unsubscribed: "subscribed".to_string(),
        sso: Some("user:alice@acme.com".to_string()),
    };
    let response: UpdateUserNotificationPageSubscriptionStatusResponse =
        update_user_notification_page_subscription_status(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]