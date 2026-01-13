## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| page | i32 | Nee |  |
| limit | i32 | Nee |  |
| skip | i32 | Nee |  |
| as_tree | bool | Nee |  |
| skip_children | i32 | Nee |  |
| limit_children | i32 | Nee |  |
| max_tree_depth | i32 | Nee |  |
| url_id | String | Nee |  |
| user_id | String | Nee |  |
| anon_user_id | String | Nee |  |
| context_user_id | String | Nee |  |
| hash_tag | String | Nee |  |
| parent_id | String | Nee |  |
| direction | models::SortDirections | Nee |  |

## Response

Geeft terug: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---