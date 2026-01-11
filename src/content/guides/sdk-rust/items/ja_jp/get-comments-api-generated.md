## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| page | i32 | いいえ |  |
| limit | i32 | いいえ |  |
| skip | i32 | いいえ |  |
| as_tree | bool | いいえ |  |
| skip_children | i32 | いいえ |  |
| limit_children | i32 | いいえ |  |
| max_tree_depth | i32 | いいえ |  |
| url_id | String | いいえ |  |
| user_id | String | いいえ |  |
| anon_user_id | String | いいえ |  |
| context_user_id | String | いいえ |  |
| hash_tag | String | いいえ |  |
| parent_id | String | いいえ |  |
| direction | models::SortDirections | いいえ |  |

## レスポンス

返却: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

---