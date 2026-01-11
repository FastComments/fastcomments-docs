## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_feed_post_params | models::CreateFeedPostParams | はい |  |
| broadcast_id | String | いいえ |  |
| is_live | bool | いいえ |  |
| do_spam_check | bool | いいえ |  |
| skip_dup_check | bool | いいえ |  |

## レスポンス

返却: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---