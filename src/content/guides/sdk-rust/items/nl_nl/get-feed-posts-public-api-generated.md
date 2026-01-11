req
tenantId
afterId

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| after_id | String | Nee |  |
| limit | i32 | Nee |  |
| tags | Vec<String> | Nee |  |
| sso | String | Nee |  |
| is_crawler | bool | Nee |  |
| include_user_info | bool | Nee |  |

## Antwoord

Geeft terug: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

---