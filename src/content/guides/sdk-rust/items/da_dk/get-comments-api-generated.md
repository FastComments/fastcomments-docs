## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| page | i32 | Nej |  |
| limit | i32 | Nej |  |
| skip | i32 | Nej |  |
| as_tree | bool | Nej |  |
| skip_children | i32 | Nej |  |
| limit_children | i32 | Nej |  |
| max_tree_depth | i32 | Nej |  |
| url_id | String | Nej |  |
| user_id | String | Nej |  |
| anon_user_id | String | Nej |  |
| context_user_id | String | Nej |  |
| hash_tag | String | Nej |  |
| parent_id | String | Nej |  |
| direction | models::SortDirections | Nej |  |

## Svar

Returnerer: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---