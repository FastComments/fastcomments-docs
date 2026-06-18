## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCount200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature';
const notificationCountNoSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId);
const notificationCountWithSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId, ssoToken);
[inline-code-end]

---