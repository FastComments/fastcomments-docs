
req
tenantId
urlId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| page | i32 | No |  |
| direction | models::SortDirections | No |  |
| sso | String | No |  |
| skip | i32 | No |  |
| skip_children | i32 | No |  |
| limit | i32 | No |  |
| limit_children | i32 | No |  |
| count_children | bool | No |  |
| fetch_page_for_comment_id | String | No |  |
| include_config | bool | No |  |
| count_all | bool | No |  |
| includei10n | bool | No |  |
| locale | String | No |  |
| modules | String | No |  |
| is_crawler | bool | No |  |
| include_notification_count | bool | No |  |
| as_tree | bool | No |  |
| max_tree_depth | i32 | No |  |
| use_full_translation_ids | bool | No |  |
| parent_id | String | No |  |
| search_text | String | No |  |
| hash_tags | Vec<String> | No |  |
| user_id | String | No |  |
| custom_config_str | String | No |  |
| after_comment_id | String | No |  |
| before_comment_id | String | No |  |

## Response

Returns: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)
