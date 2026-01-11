req
tenantId
afterId

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| after_id | String | Όχι |  |
| limit | i32 | Όχι |  |
| tags | Vec<String> | Όχι |  |
| sso | String | Όχι |  |
| is_crawler | bool | Όχι |  |
| include_user_info | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)