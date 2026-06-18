Включает или отключает уведомления для страницы. Когда пользователи подписаны на страницу, создаются уведомления
для новых корневых комментариев, а также

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| url | string | Да |  |
| pageTitle | string | Да |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример updateUserNotificationPageSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b2';
const urlId: string = 'article_987';
const url: string = 'https://www.news-site.com/articles/2026/pasta-guide';
const pageTitle: string = 'The Definitive Guide to Cooking Pasta';
const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum = UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
const sso: string = 'sso-token-62b9f1';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso);
[inline-code-end]

---