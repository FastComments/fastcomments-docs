Увімкнути або вимкнути сповіщення для сторінки. Коли користувачі підписані на сторінку, створюються сповіщення про нові кореневі коментарі, а також

## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| url | string | Так |  |
| pageTitle | string | Так |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

---