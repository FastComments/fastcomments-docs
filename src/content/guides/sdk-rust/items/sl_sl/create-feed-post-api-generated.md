## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| create_feed_post_params | models::CreateFeedPostParams | Da |  |
| broadcast_id | String | Ne |  |
| is_live | bool | Ne |  |
| do_spam_check | bool | Ne |  |
| skip_dup_check | bool | Ne |  |

## Odgovor

Vrača: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---