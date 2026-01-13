## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| post_id | String | 是 |  |
| react_body_params | models::ReactBodyParams | 是 |  |
| is_undo | bool | 否 |  |
| broadcast_id | String | 否 |  |
| sso | String | 否 |  |

## 回應

回傳： [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_public_200_response.rs)