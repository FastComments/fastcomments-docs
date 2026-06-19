Bir sayfa için bildirimleri etkinleştirin veya devre dışı bırakın. Kullanıcılar bir sayfaya abone olduklarında, yeni kök yorumlar için bildirimler oluşturulur ve ayrıca

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| url | string | Evet |  |
| pageTitle | string | Evet |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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