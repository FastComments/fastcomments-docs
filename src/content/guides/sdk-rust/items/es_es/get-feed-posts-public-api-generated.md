req
tenantId
afterId

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| after_id | String | No |  |
| limit | i32 | No |  |
| tags | Vec<String> | No |  |
| sso | String | No |  |
| is_crawler | bool | No |  |
| include_user_info | bool | No |  |

## Respuesta

Devuelve: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

---