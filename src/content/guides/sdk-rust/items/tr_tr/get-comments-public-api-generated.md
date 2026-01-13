req
tenantId
urlId

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| url_id | String | Evet |  |
| page | i32 | Hayır |  |
| direction | models::SortDirections | Hayır |  |
| sso | String | Hayır |  |
| skip | i32 | Hayır |  |
| skip_children | i32 | Hayır |  |
| limit | i32 | Hayır |  |
| limit_children | i32 | Hayır |  |
| count_children | bool | Hayır |  |
| fetch_page_for_comment_id | String | Hayır |  |
| include_config | bool | Hayır |  |
| count_all | bool | Hayır |  |
| includei10n | bool | Hayır |  |
| locale | String | Hayır |  |
| modules | String | Hayır |  |
| is_crawler | bool | Hayır |  |
| include_notification_count | bool | Hayır |  |
| as_tree | bool | Hayır |  |
| max_tree_depth | i32 | Hayır |  |
| use_full_translation_ids | bool | Hayır |  |
| parent_id | String | Hayır |  |
| search_text | String | Hayır |  |
| hash_tags | Vec<String> | Hayır |  |
| user_id | String | Hayır |  |
| custom_config_str | String | Hayır |  |
| after_comment_id | String | Hayır |  |
| before_comment_id | String | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)