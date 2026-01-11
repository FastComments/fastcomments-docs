---
## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| post_id | String | Sí |  |
| update_feed_post_params | models::UpdateFeedPostParams | Sí |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_public_200_response.rs)

---