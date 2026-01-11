## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| create_feed_post_params | models::CreateFeedPostParams | Tak |  |
| broadcast_id | String | Nie |  |
| is_live | bool | Nie |  |
| do_spam_check | bool | Nie |  |
| skip_dup_check | bool | Nie |  |

## Odpowiedź

Zwraca: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---