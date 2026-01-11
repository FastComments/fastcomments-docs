req
tenantId
afterId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| after_id | String | Nein |  |
| limit | i32 | Nein |  |
| tags | Vec<String> | Nein |  |

## Antwort

Gibt zurück: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)