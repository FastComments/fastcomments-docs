req
tenantId
urlId

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| url_id | String | Ναι |  |
| page | i32 | Όχι |  |
| direction | models::SortDirections | Όχι |  |
| sso | String | Όχι |  |
| skip | i32 | Όχι |  |
| skip_children | i32 | Όχι |  |
| limit | i32 | Όχι |  |
| limit_children | i32 | Όχι |  |
| count_children | bool | Όχι |  |
| fetch_page_for_comment_id | String | Όχι |  |
| include_config | bool | Όχι |  |
| count_all | bool | Όχι |  |
| includei10n | bool | Όχι |  |
| locale | String | Όχι |  |
| modules | String | Όχι |  |
| is_crawler | bool | Όχι |  |
| include_notification_count | bool | Όχι |  |
| as_tree | bool | Όχι |  |
| max_tree_depth | i32 | Όχι |  |
| use_full_translation_ids | bool | Όχι |  |
| parent_id | String | Όχι |  |
| search_text | String | Όχι |  |
| hash_tags | Vec<String> | Όχι |  |
| user_id | String | Όχι |  |
| custom_config_str | String | Όχι |  |
| after_comment_id | String | Όχι |  |
| before_comment_id | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)