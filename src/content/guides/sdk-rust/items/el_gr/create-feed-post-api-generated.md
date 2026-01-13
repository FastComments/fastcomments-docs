## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| create_feed_post_params | models::CreateFeedPostParams | Ναι |  |
| broadcast_id | String | Όχι |  |
| is_live | bool | Όχι |  |
| do_spam_check | bool | Όχι |  |
| skip_dup_check | bool | Όχι |  |

## Απάντηση

Επιστρέφει: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---