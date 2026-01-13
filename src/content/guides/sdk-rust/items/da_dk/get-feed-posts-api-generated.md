req
tenantId
afterId

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| after_id | String | Nej |  |
| limit | i32 | Nej |  |
| tags | Vec<String> | Nej |  |

## Svar

Returnerer: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)