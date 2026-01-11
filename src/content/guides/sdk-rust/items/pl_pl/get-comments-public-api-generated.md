req
tenantId
urlId

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| page | i32 | Nie |  |
| direction | models::SortDirections | Nie |  |
| sso | String | Nie |  |
| skip | i32 | Nie |  |
| skip_children | i32 | Nie |  |
| limit | i32 | Nie |  |
| limit_children | i32 | Nie |  |
| count_children | bool | Nie |  |
| fetch_page_for_comment_id | String | Nie |  |
| include_config | bool | Nie |  |
| count_all | bool | Nie |  |
| includei10n | bool | Nie |  |
| locale | String | Nie |  |
| modules | String | Nie |  |
| is_crawler | bool | Nie |  |
| include_notification_count | bool | Nie |  |
| as_tree | bool | Nie |  |
| max_tree_depth | i32 | Nie |  |
| use_full_translation_ids | bool | Nie |  |
| parent_id | String | Nie |  |
| search_text | String | Nie |  |
| hash_tags | Vec<String> | Nie |  |
| user_id | String | Nie |  |
| custom_config_str | String | Nie |  |
| after_comment_id | String | Nie |  |
| before_comment_id | String | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)