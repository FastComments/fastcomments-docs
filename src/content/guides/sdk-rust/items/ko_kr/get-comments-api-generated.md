## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| page | i32 | 아니요 |  |
| limit | i32 | 아니요 |  |
| skip | i32 | 아니요 |  |
| as_tree | bool | 아니요 |  |
| skip_children | i32 | 아니요 |  |
| limit_children | i32 | 아니요 |  |
| max_tree_depth | i32 | 아니요 |  |
| url_id | String | 아니요 |  |
| user_id | String | 아니요 |  |
| anon_user_id | String | 아니요 |  |
| context_user_id | String | 아니요 |  |
| hash_tag | String | 아니요 |  |
| parent_id | String | 아니요 |  |
| direction | models::SortDirections | 아니요 |  |
| from_date | i64 | 아니요 |  |
| to_date | i64 | 아니요 |  |

## 응답

반환: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## 예제

[inline-code-attrs-start title = 'get_comments 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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