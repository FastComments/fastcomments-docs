## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenant_id | String | Oui |  |
| create_feed_post_params | models::CreateFeedPostParams | Oui |  |
| broadcast_id | String | Non |  |
| is_live | bool | Non |  |
| do_spam_check | bool | Non |  |
| skip_dup_check | bool | Non |  |

## Réponse

Renvoie : [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---