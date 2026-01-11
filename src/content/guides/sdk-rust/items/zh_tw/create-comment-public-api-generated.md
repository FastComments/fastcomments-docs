## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| comment_data | models::CommentData | 是 |  |
| session_id | String | 否 |  |
| sso | String | 否 |  |

## 回應

回傳： [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_comment_public_200_response.rs)

---