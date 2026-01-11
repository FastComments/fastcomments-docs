## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_feed_post_params | models::CreateFeedPostParams | Ja |  |
| broadcast_id | String | Nej |  |
| is_live | bool | Nej |  |
| do_spam_check | bool | Nej |  |
| skip_dup_check | bool | Nej |  |

## Svar

Returnerer: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---