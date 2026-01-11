## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| create_feed_post_params | models::CreateFeedPostParams | Sí |  |
| broadcast_id | String | No |  |
| is_live | bool | No |  |
| do_spam_check | bool | No |  |
| skip_dup_check | bool | No |  |

## Respuesta

Devuelve: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---