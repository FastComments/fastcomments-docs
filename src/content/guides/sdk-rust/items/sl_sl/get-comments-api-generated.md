## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| page | i32 | Ne |  |
| limit | i32 | Ne |  |
| skip | i32 | Ne |  |
| as_tree | bool | Ne |  |
| skip_children | i32 | Ne |  |
| limit_children | i32 | Ne |  |
| max_tree_depth | i32 | Ne |  |
| url_id | String | Ne |  |
| user_id | String | Ne |  |
| anon_user_id | String | Ne |  |
| context_user_id | String | Ne |  |
| hash_tag | String | Ne |  |
| parent_id | String | Ne |  |
| direction | models::SortDirections | Ne |  |

## Odgovor

Vrne: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---