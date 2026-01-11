---
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

## Απόκριση

Επιστρέφει: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)

---