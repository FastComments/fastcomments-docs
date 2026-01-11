---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| page | i32 | Ні |  |
| limit | i32 | Ні |  |
| skip | i32 | Ні |  |
| as_tree | bool | Ні |  |
| skip_children | i32 | Ні |  |
| limit_children | i32 | Ні |  |
| max_tree_depth | i32 | Ні |  |
| url_id | String | Ні |  |
| user_id | String | Ні |  |
| anon_user_id | String | Ні |  |
| context_user_id | String | Ні |  |
| hash_tag | String | Ні |  |
| parent_id | String | Ні |  |
| direction | models::SortDirections | Ні |  |

## Відповідь

Повертає: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---