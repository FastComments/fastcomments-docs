## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| page | i32 | Не |  |
| limit | i32 | Не |  |
| skip | i32 | Не |  |
| as_tree | bool | Не |  |
| skip_children | i32 | Не |  |
| limit_children | i32 | Не |  |
| max_tree_depth | i32 | Не |  |
| url_id | String | Не |  |
| user_id | String | Не |  |
| anon_user_id | String | Не |  |
| context_user_id | String | Не |  |
| hash_tag | String | Не |  |
| parent_id | String | Не |  |
| direction | models::SortDirections | Не |  |

## Отговор

Връща: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---