## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| page | i32 | 否 |  |
| limit | i32 | 否 |  |
| skip | i32 | 否 |  |
| as_tree | bool | 否 |  |
| skip_children | i32 | 否 |  |
| limit_children | i32 | 否 |  |
| max_tree_depth | i32 | 否 |  |
| url_id | String | 否 |  |
| user_id | String | 否 |  |
| anon_user_id | String | 否 |  |
| context_user_id | String | 否 |  |
| hash_tag | String | 否 |  |
| parent_id | String | 否 |  |
| direction | models::SortDirections | 否 |  |

## 响应

返回：[`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---