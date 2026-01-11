---
req
tenantId
afterId

## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| after_id | String | Ne |  |
| limit | i32 | Ne |  |
| tags | Vec<String> | Ne |  |
| sso | String | Ne |  |
| is_crawler | bool | Ne |  |
| include_user_info | bool | Ne |  |

## Odgovor

Vrača: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

---