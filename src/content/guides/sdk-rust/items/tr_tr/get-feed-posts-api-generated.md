req
tenantId
afterId

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| after_id | String | Hayır |  |
| limit | i32 | Hayır |  |
| tags | Vec<String> | Hayır |  |

## Yanıt

Döndürür: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)

---