---
Schakel meldingen in of uit voor een pagina. Wanneer gebruikers op een pagina geabonneerd zijn, worden er meldingen aangemaakt
voor nieuwe root-opmerkingen, en ook

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| url | string | Ja |  |
| pageTitle | string | Ja |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Ja |  |
| sso | string | Nee |  |

## Response

Retourneert: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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