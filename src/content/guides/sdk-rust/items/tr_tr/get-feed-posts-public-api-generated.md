req
tenantId
afterId

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| after_id | String | Hayır |  |
| limit | i32 | Hayır |  |
| tags | Vec<String> | Hayır |  |
| sso | String | Hayır |  |
| is_crawler | bool | Hayır |  |
| include_user_info | bool | Hayır |  |

## Yanıt

Döndürür: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)