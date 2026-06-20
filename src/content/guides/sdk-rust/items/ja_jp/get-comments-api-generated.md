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
| from_date | i64 | いいえ |  |
| to_date | i64 | いいえ |  |

## レスポンス

戻り値: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## 例

[inline-code-attrs-start title = 'get_comments の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetCommentsParams = GetCommentsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1),
        limit: Some(25),
        skip: Some(0),
        as_tree: Some(true),
        skip_children: Some(0),
        limit_children: Some(5),
        max_tree_depth: Some(3),
        url_id: Some("news/article/2026/06/fast-rust".to_string()),
        user_id: Some("user-1234".to_string()),
        anon_user_id: Some("anon-5678".to_string()),
        context_user_id: Some("context-999".to_string()),
        hash_tag: Some("release".to_string()),
        parent_id: Some("comment-9876".to_string()),
        direction: Some(models::SortDirections::Desc),
        from_date: Some(1_689_000_000_i64),
        to_date: Some(1_689_086_400_i64),
    };

    let response: ApiGetCommentsResponse = get_comments(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---