req
tenantId
urlId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| page | i32 | Nee |  |
| direction | models::SortDirections | Nee |  |
| sso | String | Nee |  |
| skip | i32 | Nee |  |
| skip_children | i32 | Nee |  |
| limit | i32 | Nee |  |
| limit_children | i32 | Nee |  |
| count_children | bool | Nee |  |
| fetch_page_for_comment_id | String | Nee |  |
| include_config | bool | Nee |  |
| count_all | bool | Nee |  |
| includei10n | bool | Nee |  |
| locale | String | Nee |  |
| modules | String | Nee |  |
| is_crawler | bool | Nee |  |
| include_notification_count | bool | Nee |  |
| as_tree | bool | Nee |  |
| max_tree_depth | i32 | Nee |  |
| use_full_translation_ids | bool | Nee |  |
| parent_id | String | Nee |  |
| search_text | String | Nee |  |
| hash_tags | Vec<String> | Nee |  |
| user_id | String | Nee |  |
| custom_config_str | String | Nee |  |
| after_comment_id | String | Nee |  |
| before_comment_id | String | Nee |  |

## Respons

Geeft terug: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)