## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| page | i32 | Όχι |  |
| limit | i32 | Όχι |  |
| skip | i32 | Όχι |  |
| as_tree | bool | Όχι |  |
| skip_children | i32 | Όχι |  |
| limit_children | i32 | Όχι |  |
| max_tree_depth | i32 | Όχι |  |
| url_id | String | Όχι |  |
| user_id | String | Όχι |  |
| anon_user_id | String | Όχι |  |
| context_user_id | String | Όχι |  |
| hash_tag | String | Όχι |  |
| parent_id | String | Όχι |  |
| direction | models::SortDirections | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---