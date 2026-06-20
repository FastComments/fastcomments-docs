## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| page | i32 | No |  |
| limit | i32 | No |  |
| skip | i32 | No |  |
| as_tree | bool | No |  |
| skip_children | i32 | No |  |
| limit_children | i32 | No |  |
| max_tree_depth | i32 | No |  |
| url_id | String | No |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |
| context_user_id | String | No |  |
| hash_tag | String | No |  |
| parent_id | String | No |  |
| direction | models::SortDirections | No |  |
| from_date | i64 | No |  |
| to_date | i64 | No |  |

## Yanıt

Döndürür: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_comments Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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