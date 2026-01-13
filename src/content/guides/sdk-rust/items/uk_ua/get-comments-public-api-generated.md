req
tenantId
urlId

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| page | i32 | Ні |  |
| direction | models::SortDirections | Ні |  |
| sso | String | Ні |  |
| skip | i32 | Ні |  |
| skip_children | i32 | Ні |  |
| limit | i32 | Ні |  |
| limit_children | i32 | Ні |  |
| count_children | bool | Ні |  |
| fetch_page_for_comment_id | String | Ні |  |
| include_config | bool | Ні |  |
| count_all | bool | Ні |  |
| includei10n | bool | Ні |  |
| locale | String | Ні |  |
| modules | String | Ні |  |
| is_crawler | bool | Ні |  |
| include_notification_count | bool | Ні |  |
| as_tree | bool | Ні |  |
| max_tree_depth | i32 | Ні |  |
| use_full_translation_ids | bool | Ні |  |
| parent_id | String | Ні |  |
| search_text | String | Ні |  |
| hash_tags | Vec<String> | Ні |  |
| user_id | String | Ні |  |
| custom_config_str | String | Ні |  |
| after_comment_id | String | Ні |  |
| before_comment_id | String | Ні |  |

## Відповідь

Повертає: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

---