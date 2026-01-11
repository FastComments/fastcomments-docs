## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| post_id | String | Da |  |
| update_feed_post_params | models::UpdateFeedPostParams | Da |  |
| broadcast_id | String | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_public_200_response.rs)