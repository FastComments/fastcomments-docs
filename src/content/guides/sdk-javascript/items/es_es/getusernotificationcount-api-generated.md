## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_3f47a2b9-6c4d-4e8a-9f2b-0a1b2c3d4e5f';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4OTAiLCJlbWFpbCI6InVzZXJAZXhhbXBsZS5jb20ifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const notificationCount: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId);
const notificationCountWithSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId, ssoToken);
[inline-code-end]

---