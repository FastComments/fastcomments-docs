---
Увімкнути або вимкнути сповіщення для сторінки. Коли користувачі підписані на сторінку, створюються сповіщення про нові кореневі коментарі, а також

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| url | String | Так |  |
| page_title | String | Так |  |
| subscribed_or_unsubscribed | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---