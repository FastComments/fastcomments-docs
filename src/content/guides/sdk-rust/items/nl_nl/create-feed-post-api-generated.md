## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_feed_post_params | models::CreateFeedPostParams | Ja |  |
| broadcast_id | String | Nee |  |
| is_live | bool | Nee |  |
| do_spam_check | bool | Nee |  |
| skip_dup_check | bool | Nee |  |

## Respons

Retourneert: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)