## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| page | i32 | Nie |  |
| limit | i32 | Nie |  |
| skip | i32 | Nie |  |
| as_tree | bool | Nie |  |
| skip_children | i32 | Nie |  |
| limit_children | i32 | Nie |  |
| max_tree_depth | i32 | Nie |  |
| url_id | String | Nie |  |
| user_id | String | Nie |  |
| anon_user_id | String | Nie |  |
| context_user_id | String | Nie |  |
| hash_tag | String | Nie |  |
| parent_id | String | Nie |  |
| direction | models::SortDirections | Nie |  |

## Odpowiedź

Zwraca: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---