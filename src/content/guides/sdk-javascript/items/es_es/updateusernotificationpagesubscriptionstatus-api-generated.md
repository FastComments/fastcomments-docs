Habilitar o deshabilitar las notificaciones para una página. Cuando los usuarios están suscritos a una página, se crean
notificaciones para nuevos comentarios raíz, y también

## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| url | string | Sí |  |
| pageTitle | string | Sí |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateUserNotificationPageSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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