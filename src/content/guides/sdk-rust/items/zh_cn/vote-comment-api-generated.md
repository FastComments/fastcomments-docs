## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| url_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| vote_body_params | models::VoteBodyParams | 是 |  |
| session_id | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回：[`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

---