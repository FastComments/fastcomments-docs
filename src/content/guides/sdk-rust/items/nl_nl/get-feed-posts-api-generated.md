req
tenantId
afterId

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| after_id | String | Nee |  |
| limit | i32 | Nee |  |
| tags | Vec<String> | Nee |  |

## Respons

Retourneert: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)