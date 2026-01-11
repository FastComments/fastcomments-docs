req
tenantId
urlId

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| url_id | String | כן |  |
| page | i32 | לא |  |
| direction | models::SortDirections | לא |  |
| sso | String | לא |  |
| skip | i32 | לא |  |
| skip_children | i32 | לא |  |
| limit | i32 | לא |  |
| limit_children | i32 | לא |  |
| count_children | bool | לא |  |
| fetch_page_for_comment_id | String | לא |  |
| include_config | bool | לא |  |
| count_all | bool | לא |  |
| includei10n | bool | לא |  |
| locale | String | לא |  |
| modules | String | לא |  |
| is_crawler | bool | לא |  |
| include_notification_count | bool | לא |  |
| as_tree | bool | לא |  |
| max_tree_depth | i32 | לא |  |
| use_full_translation_ids | bool | לא |  |
| parent_id | String | לא |  |
| search_text | String | לא |  |
| hash_tags | Vec<String> | לא |  |
| user_id | String | לא |  |
| custom_config_str | String | לא |  |
| after_comment_id | String | לא |  |
| before_comment_id | String | לא |  |

## תגובה

מחזיר: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

---