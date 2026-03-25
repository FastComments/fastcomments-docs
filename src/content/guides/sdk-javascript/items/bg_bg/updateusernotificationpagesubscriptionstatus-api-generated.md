Активиране или деактивиране на уведомления за страница. Когато потребителите са абонирани за страница, се създават уведомления за нови коренови коментари, както и

## Parameters

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| url | string | Да |  |
| pageTitle | string | Да |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Да |  |
| sso | string | Не |  |

## Отговор

Връща: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Пример

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const urlId: string = 'blog-launch-2026';
const url: string = 'https://acme.example.com/blog/launch-march-2026';
const pageTitle: string = 'Acme Product Launch — March 2026';
const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum = UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
const sso: string = 'sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const response: UpdateUserNotificationStatus200Response = await updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso);
[inline-code-end]

---