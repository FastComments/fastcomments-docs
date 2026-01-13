req
tenantId
urlId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| page | i32 | Nein |  |
| direction | models::SortDirections | Nein |  |
| sso | String | Nein |  |
| skip | i32 | Nein |  |
| skip_children | i32 | Nein |  |
| limit | i32 | Nein |  |
| limit_children | i32 | Nein |  |
| count_children | bool | Nein |  |
| fetch_page_for_comment_id | String | Nein |  |
| include_config | bool | Nein |  |
| count_all | bool | Nein |  |
| includei10n | bool | Nein |  |
| locale | String | Nein |  |
| modules | String | Nein |  |
| is_crawler | bool | Nein |  |
| include_notification_count | bool | Nein |  |
| as_tree | bool | Nein |  |
| max_tree_depth | i32 | Nein |  |
| use_full_translation_ids | bool | Nein |  |
| parent_id | String | Nein |  |
| search_text | String | Nein |  |
| hash_tags | Vec<String> | Nein |  |
| user_id | String | Nein |  |
| custom_config_str | String | Nein |  |
| after_comment_id | String | Nein |  |
| before_comment_id | String | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)