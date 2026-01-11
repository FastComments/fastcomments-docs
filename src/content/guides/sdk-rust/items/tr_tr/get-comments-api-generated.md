## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| page | i32 | Hayır |  |
| limit | i32 | Hayır |  |
| skip | i32 | Hayır |  |
| as_tree | bool | Hayır |  |
| skip_children | i32 | Hayır |  |
| limit_children | i32 | Hayır |  |
| max_tree_depth | i32 | Hayır |  |
| url_id | String | Hayır |  |
| user_id | String | Hayır |  |
| anon_user_id | String | Hayır |  |
| context_user_id | String | Hayır |  |
| hash_tag | String | Hayır |  |
| parent_id | String | Hayır |  |
| direction | models::SortDirections | Hayır |  |

## Yanıt

Döndürür: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---