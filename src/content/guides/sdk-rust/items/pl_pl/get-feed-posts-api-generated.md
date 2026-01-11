---
req
tenantId
afterId

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| after_id | String | Nie |  |
| limit | i32 | Nie |  |
| tags | Vec<String> | Nie |  |

## Odpowiedź

Zwraca: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)

---