## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| page | i32 | לא |  |
| limit | i32 | לא |  |
| skip | i32 | לא |  |
| as_tree | bool | לא |  |
| skip_children | i32 | לא |  |
| limit_children | i32 | לא |  |
| max_tree_depth | i32 | לא |  |
| url_id | String | לא |  |
| user_id | String | לא |  |
| anon_user_id | String | לא |  |
| context_user_id | String | לא |  |
| hash_tag | String | לא |  |
| parent_id | String | לא |  |
| direction | models::SortDirections | לא |  |

## תגובה

מחזיר: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---