req
tenantId
urlId

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| page | i32 | Non |  |
| direction | models::SortDirections | Non |  |
| sso | String | Non |  |
| skip | i32 | Non |  |
| skip_children | i32 | Non |  |
| limit | i32 | Non |  |
| limit_children | i32 | Non |  |
| count_children | bool | Non |  |
| fetch_page_for_comment_id | String | Non |  |
| include_config | bool | Non |  |
| count_all | bool | Non |  |
| includei10n | bool | Non |  |
| locale | String | Non |  |
| modules | String | Non |  |
| is_crawler | bool | Non |  |
| include_notification_count | bool | Non |  |
| as_tree | bool | Non |  |
| max_tree_depth | i32 | Non |  |
| use_full_translation_ids | bool | Non |  |
| parent_id | String | Non |  |
| search_text | String | Non |  |
| hash_tags | Vec<String> | Non |  |
| user_id | String | Non |  |
| custom_config_str | String | Non |  |
| after_comment_id | String | Non |  |
| before_comment_id | String | Non |  |

## Réponse

Renvoie: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)