## Parâmetros

| Name | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| page | i32 | Não |  |
| limit | i32 | Não |  |
| skip | i32 | Não |  |
| as_tree | bool | Não |  |
| skip_children | i32 | Não |  |
| limit_children | i32 | Não |  |
| max_tree_depth | i32 | Não |  |
| url_id | String | Não |  |
| user_id | String | Não |  |
| anon_user_id | String | Não |  |
| context_user_id | String | Não |  |
| hash_tag | String | Não |  |
| parent_id | String | Não |  |
| direction | models::SortDirections | Não |  |

## Resposta

Retorna: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)