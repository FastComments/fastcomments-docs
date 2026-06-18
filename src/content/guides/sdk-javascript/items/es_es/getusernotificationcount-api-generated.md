## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCount200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature';
const notificationCountNoSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId);
const notificationCountWithSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId, ssoToken);
[inline-code-end]

---