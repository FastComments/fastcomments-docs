req
tenantId
urlId

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| page | i32 | Ne |  |
| direction | models::SortDirections | Ne |  |
| sso | String | Ne |  |
| skip | i32 | Ne |  |
| skip_children | i32 | Ne |  |
| limit | i32 | Ne |  |
| limit_children | i32 | Ne |  |
| count_children | bool | Ne |  |
| fetch_page_for_comment_id | String | Ne |  |
| include_config | bool | Ne |  |
| count_all | bool | Ne |  |
| includei10n | bool | Ne |  |
| locale | String | Ne |  |
| modules | String | Ne |  |
| is_crawler | bool | Ne |  |
| include_notification_count | bool | Ne |  |
| as_tree | bool | Ne |  |
| max_tree_depth | i32 | Ne |  |
| use_full_translation_ids | bool | Ne |  |
| parent_id | String | Ne |  |
| search_text | String | Ne |  |
| hash_tags | Vec<String> | Ne |  |
| user_id | String | Ne |  |
| custom_config_str | String | Ne |  |
| after_comment_id | String | Ne |  |
| before_comment_id | String | Ne |  |

## Odgovor

Vraća: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)