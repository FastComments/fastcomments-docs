req
tenantId
urlId

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| page | i32 | 아니오 |  |
| direction | models::SortDirections | 아니오 |  |
| sso | String | 아니오 |  |
| skip | i32 | 아니오 |  |
| skip_children | i32 | 아니오 |  |
| limit | i32 | 아니오 |  |
| limit_children | i32 | 아니오 |  |
| count_children | bool | 아니오 |  |
| fetch_page_for_comment_id | String | 아니오 |  |
| include_config | bool | 아니오 |  |
| count_all | bool | 아니오 |  |
| includei10n | bool | 아니오 |  |
| locale | String | 아니오 |  |
| modules | String | 아니오 |  |
| is_crawler | bool | 아니오 |  |
| include_notification_count | bool | 아니오 |  |
| as_tree | bool | 아니오 |  |
| max_tree_depth | i32 | 아니오 |  |
| use_full_translation_ids | bool | 아니오 |  |
| parent_id | String | 아니오 |  |
| search_text | String | 아니오 |  |
| hash_tags | Vec<String> | 아니오 |  |
| user_id | String | 아니오 |  |
| custom_config_str | String | 아니오 |  |
| after_comment_id | String | 아니오 |  |
| before_comment_id | String | 아니오 |  |

## 응답

반환: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)