---
Habilitar o deshabilitar notificaciones para un comentario específico.

## Parámetros

| Nombre | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| notification_id | String | Sí |  |
| opted_in_or_out | String | Sí |  |
| comment_id | String | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---