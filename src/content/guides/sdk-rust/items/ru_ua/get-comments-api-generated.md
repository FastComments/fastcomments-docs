## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| page | i32 | Нет |  |
| limit | i32 | Нет |  |
| skip | i32 | Нет |  |
| as_tree | bool | Нет |  |
| skip_children | i32 | Нет |  |
| limit_children | i32 | Нет |  |
| max_tree_depth | i32 | Нет |  |
| url_id | String | Нет |  |
| user_id | String | Нет |  |
| anon_user_id | String | Нет |  |
| context_user_id | String | Нет |  |
| hash_tag | String | Нет |  |
| parent_id | String | Нет |  |
| direction | models::SortDirections | Нет |  |

## Ответ

Возвращает: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---