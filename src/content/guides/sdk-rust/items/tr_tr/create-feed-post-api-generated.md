## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| create_feed_post_params | models::CreateFeedPostParams | Evet |  |
| broadcast_id | String | Hayır |  |
| is_live | bool | Hayır |  |
| do_spam_check | bool | Hayır |  |
| skip_dup_check | bool | Hayır |  |

## Yanıt

Döndürür: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---