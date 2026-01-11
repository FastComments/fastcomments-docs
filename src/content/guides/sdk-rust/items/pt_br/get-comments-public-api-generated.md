req
tenantId
urlId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| page | i32 | Não |  |
| direction | models::SortDirections | Não |  |
| sso | String | Não |  |
| skip | i32 | Não |  |
| skip_children | i32 | Não |  |
| limit | i32 | Não |  |
| limit_children | i32 | Não |  |
| count_children | bool | Não |  |
| fetch_page_for_comment_id | String | Não |  |
| include_config | bool | Não |  |
| count_all | bool | Não |  |
| includei10n | bool | Não |  |
| locale | String | Não |  |
| modules | String | Não |  |
| is_crawler | bool | Não |  |
| include_notification_count | bool | Não |  |
| as_tree | bool | Não |  |
| max_tree_depth | i32 | Não |  |
| use_full_translation_ids | bool | Não |  |
| parent_id | String | Não |  |
| search_text | String | Não |  |
| hash_tags | Vec<String> | Não |  |
| user_id | String | Não |  |
| custom_config_str | String | Não |  |
| after_comment_id | String | Não |  |
| before_comment_id | String | Não |  |

## Resposta

Retorna: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)