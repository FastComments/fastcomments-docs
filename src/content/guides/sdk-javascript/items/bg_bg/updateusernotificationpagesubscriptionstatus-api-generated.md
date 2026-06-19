Разрешаване или забраняване на известия за страница. Когато потребителите са абонирани за страница, се създават известия за нови основни коментари, и също

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| url | string | Да |  |
| pageTitle | string | Да |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Да |  |
| sso | string | Не |  |

## Отговор

Връща: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за updateUserNotificationPageSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "8a3f2b1c-4d6e-4f9b-9c2d-0a1b2c3d4e5f";
const urlId: string = "article-2026-reliable-api";
const url: string = "https://blog.companyexample.com/articles/reliable-api-patterns";
const pageTitle: string = "Reliable API Patterns for Integrations";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload";
const result: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
  tenantId,
  urlId,
  url,
  pageTitle,
  UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed,
  sso
);
[inline-code-end]

---