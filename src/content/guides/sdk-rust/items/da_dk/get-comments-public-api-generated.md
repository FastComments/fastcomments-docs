req
tenantId
urlId

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| page | i32 | Nej |  |
| direction | models::SortDirections | Nej |  |
| sso | String | Nej |  |
| skip | i32 | Nej |  |
| skip_children | i32 | Nej |  |
| limit | i32 | Nej |  |
| limit_children | i32 | Nej |  |
| count_children | bool | Nej |  |
| fetch_page_for_comment_id | String | Nej |  |
| include_config | bool | Nej |  |
| count_all | bool | Nej |  |
| includei10n | bool | Nej |  |
| locale | String | Nej |  |
| modules | String | Nej |  |
| is_crawler | bool | Nej |  |
| include_notification_count | bool | Nej |  |
| as_tree | bool | Nej |  |
| max_tree_depth | i32 | Nej |  |
| use_full_translation_ids | bool | Nej |  |
| parent_id | String | Nej |  |
| search_text | String | Nej |  |
| hash_tags | Vec<String> | Nej |  |
| user_id | String | Nej |  |
| custom_config_str | String | Nej |  |
| after_comment_id | String | Nej |  |
| before_comment_id | String | Nej |  |

## Svar

Returnerer: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)