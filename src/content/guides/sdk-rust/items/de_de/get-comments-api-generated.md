## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| page | i32 | Nein |  |
| limit | i32 | Nein |  |
| skip | i32 | Nein |  |
| as_tree | bool | Nein |  |
| skip_children | i32 | Nein |  |
| limit_children | i32 | Nein |  |
| max_tree_depth | i32 | Nein |  |
| url_id | String | Nein |  |
| user_id | String | Nein |  |
| anon_user_id | String | Nein |  |
| context_user_id | String | Nein |  |
| hash_tag | String | Nein |  |
| parent_id | String | Nein |  |
| direction | models::SortDirections | Nein |  |

## Antwort

Gibt zurück: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---