Habilitar o deshabilitar las notificaciones para una página. Cuando los usuarios están suscritos a una página, se crean notificaciones para nuevos comentarios raíz, y también

## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| url_id | String | Sí |  |
| url | String | Sí |  |
| page_title | String | Sí |  |
| subscribed_or_unsubscribed | String | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---