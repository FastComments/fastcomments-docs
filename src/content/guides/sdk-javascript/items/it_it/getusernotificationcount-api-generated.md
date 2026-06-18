## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCount200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature';
const notificationCountNoSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId);
const notificationCountWithSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId, ssoToken);
[inline-code-end]

---