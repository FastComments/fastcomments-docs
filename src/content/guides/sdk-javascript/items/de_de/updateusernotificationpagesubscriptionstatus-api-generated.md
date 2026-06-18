Benachrichtigungen für eine Seite aktivieren oder deaktivieren. Wenn Benutzer für eine Seite abonniert sind, werden Benachrichtigungen
für neue Root-Kommentare erstellt, und auch

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| url | string | Ja |  |
| pageTitle | string | Ja |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Ja |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für updateUserNotificationPageSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b2';
const urlId: string = 'article_987';
const url: string = 'https://www.news-site.com/articles/2026/pasta-guide';
const pageTitle: string = 'The Definitive Guide to Cooking Pasta';
const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum = UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
const sso: string = 'sso-token-62b9f1';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso);
[inline-code-end]