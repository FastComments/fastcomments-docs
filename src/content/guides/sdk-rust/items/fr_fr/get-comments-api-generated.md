## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| page | i32 | Non |  |
| limit | i32 | Non |  |
| skip | i32 | Non |  |
| as_tree | bool | Non |  |
| skip_children | i32 | Non |  |
| limit_children | i32 | Non |  |
| max_tree_depth | i32 | Non |  |
| url_id | String | Non |  |
| user_id | String | Non |  |
| anon_user_id | String | Non |  |
| context_user_id | String | Non |  |
| hash_tag | String | Non |  |
| parent_id | String | Non |  |
| direction | models::SortDirections | Non |  |

## Réponse

Renvoie: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---