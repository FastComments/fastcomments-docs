req
tenantId
urlId

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| page | i32 | 否 |  |
| direction | models::SortDirections | 否 |  |
| sso | String | 否 |  |
| skip | i32 | 否 |  |
| skip_children | i32 | 否 |  |
| limit | i32 | 否 |  |
| limit_children | i32 | 否 |  |
| count_children | bool | 否 |  |
| fetch_page_for_comment_id | String | 否 |  |
| include_config | bool | 否 |  |
| count_all | bool | 否 |  |
| includei10n | bool | 否 |  |
| locale | String | 否 |  |
| modules | String | 否 |  |
| is_crawler | bool | 否 |  |
| include_notification_count | bool | 否 |  |
| as_tree | bool | 否 |  |
| max_tree_depth | i32 | 否 |  |
| use_full_translation_ids | bool | 否 |  |
| parent_id | String | 否 |  |
| search_text | String | 否 |  |
| hash_tags | Vec<String> | 否 |  |
| user_id | String | 否 |  |
| custom_config_str | String | 否 |  |
| after_comment_id | String | 否 |  |
| before_comment_id | String | 否 |  |

## 回應

回傳：[`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)