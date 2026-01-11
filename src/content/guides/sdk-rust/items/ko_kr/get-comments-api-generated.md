---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| page | i32 | 아니요 |  |
| limit | i32 | 아니요 |  |
| skip | i32 | 아니요 |  |
| as_tree | bool | 아니요 |  |
| skip_children | i32 | 아니요 |  |
| limit_children | i32 | 아니요 |  |
| max_tree_depth | i32 | 아니요 |  |
| url_id | String | 아니요 |  |
| user_id | String | 아니요 |  |
| anon_user_id | String | 아니요 |  |
| context_user_id | String | 아니요 |  |
| hash_tag | String | 아니요 |  |
| parent_id | String | 아니요 |  |
| direction | models::SortDirections | 아니요 |  |

## 응답

반환: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---