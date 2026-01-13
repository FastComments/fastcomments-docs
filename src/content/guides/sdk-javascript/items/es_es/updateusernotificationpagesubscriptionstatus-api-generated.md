---
Habilitar o deshabilitar las notificaciones para una página. Cuando los usuarios están suscritos a una página, se crean notificaciones
para nuevos comentarios raíz, y también

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| url | string | Sí |  |
| pageTitle | string | Sí |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

---