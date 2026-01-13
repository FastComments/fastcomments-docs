---
req
tenantId
urlId

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| url_id | String | はい |  |
| page | i32 | いいえ |  |
| direction | models::SortDirections | いいえ |  |
| sso | String | いいえ |  |
| skip | i32 | いいえ |  |
| skip_children | i32 | いいえ |  |
| limit | i32 | いいえ |  |
| limit_children | i32 | いいえ |  |
| count_children | bool | いいえ |  |
| fetch_page_for_comment_id | String | いいえ |  |
| include_config | bool | いいえ |  |
| count_all | bool | いいえ |  |
| includei10n | bool | いいえ |  |
| locale | String | いいえ |  |
| modules | String | いいえ |  |
| is_crawler | bool | いいえ |  |
| include_notification_count | bool | いいえ |  |
| as_tree | bool | いいえ |  |
| max_tree_depth | i32 | いいえ |  |
| use_full_translation_ids | bool | いいえ |  |
| parent_id | String | いいえ |  |
| search_text | String | いいえ |  |
| hash_tags | Vec<String> | いいえ |  |
| user_id | String | いいえ |  |
| custom_config_str | String | いいえ |  |
| after_comment_id | String | いいえ |  |
| before_comment_id | String | いいえ |  |

## レスポンス

戻り値: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

---