req
tenantId
afterId

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| after_id | String | Não |  |
| limit | i32 | Não |  |
| tags | Vec<String> | Não |  |
| sso | String | Não |  |
| is_crawler | bool | Não |  |
| include_user_info | bool | Não |  |

## Resposta

Retorna: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)