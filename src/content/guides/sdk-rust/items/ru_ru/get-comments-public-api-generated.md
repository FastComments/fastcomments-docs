req
tenantId
urlId

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| page | i32 | Нет |  |
| direction | models::SortDirections | Нет |  |
| sso | String | Нет |  |
| skip | i32 | Нет |  |
| skip_children | i32 | Нет |  |
| limit | i32 | Нет |  |
| limit_children | i32 | Нет |  |
| count_children | bool | Нет |  |
| fetch_page_for_comment_id | String | Нет |  |
| include_config | bool | Нет |  |
| count_all | bool | Нет |  |
| includei10n | bool | Нет |  |
| locale | String | Нет |  |
| modules | String | Нет |  |
| is_crawler | bool | Нет |  |
| include_notification_count | bool | Нет |  |
| as_tree | bool | Нет |  |
| max_tree_depth | i32 | Нет |  |
| use_full_translation_ids | bool | Нет |  |
| parent_id | String | Нет |  |
| search_text | String | Нет |  |
| hash_tags | Vec<String> | Нет |  |
| user_id | String | Нет |  |
| custom_config_str | String | Нет |  |
| after_comment_id | String | Нет |  |
| before_comment_id | String | Нет |  |

## Ответ

Возвращает: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

---