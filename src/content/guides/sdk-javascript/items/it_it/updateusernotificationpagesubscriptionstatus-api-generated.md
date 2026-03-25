Abilita o disabilita le notifiche per una pagina. Quando gli utenti sono iscritti a una pagina, vengono create
notifiche per i nuovi commenti principali, e anche

## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| url | string | Sì |  |
| pageTitle | string | Sì |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Esempio

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Esempio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const urlId: string = 'blog-launch-2026';
const url: string = 'https://acme.example.com/blog/launch-march-2026';
const pageTitle: string = 'Acme Product Launch — March 2026';
const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum = UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
const sso: string = 'sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const response: UpdateUserNotificationStatus200Response = await updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso);
[inline-code-end]