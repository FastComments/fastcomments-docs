req
tenantId
afterId

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| after_id | String | Nej |  |
| limit | i32 | Nej |  |
| tags | Vec<String> | Nej |  |
| sso | String | Nej |  |
| is_crawler | bool | Nej |  |
| include_user_info | bool | Nej |  |

## Svar

Returnerer: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)