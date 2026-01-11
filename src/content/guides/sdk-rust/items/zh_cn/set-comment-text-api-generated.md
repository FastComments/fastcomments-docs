---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| comment_text_update_request | models::CommentTextUpdateRequest | 是 |  |
| edit_key | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_200_response.rs)

---