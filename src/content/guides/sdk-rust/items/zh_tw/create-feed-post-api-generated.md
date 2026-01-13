## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| create_feed_post_params | models::CreateFeedPostParams | 是 |  |
| broadcast_id | String | 否 |  |
| is_live | bool | 否 |  |
| do_spam_check | bool | 否 |  |
| skip_dup_check | bool | 否 |  |

## 回應

回傳: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)