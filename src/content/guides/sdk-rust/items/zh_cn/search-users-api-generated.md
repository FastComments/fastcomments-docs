---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| username_starts_with | String | 是 |  |
| mention_group_ids | Vec<String> | 否 |  |
| sso | String | 否 |  |

## 响应

返回：[`SearchUsers200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_200_response.rs)

---