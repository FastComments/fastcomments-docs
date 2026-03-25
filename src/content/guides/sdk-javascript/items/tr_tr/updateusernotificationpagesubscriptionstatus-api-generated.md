Bir sayfa için bildirimleri etkinleştirin veya devre dışı bırakın. Kullanıcılar bir sayfaya abone olduğunda, yeni root yorumlar için bildirimler oluşturulur ve ayrıca

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| url | string | Evet |  |
| pageTitle | string | Evet |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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