---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| page | i32 | 否 |  |
| limit | i32 | 否 |  |
| skip | i32 | 否 |  |
| as_tree | bool | 否 |  |
| skip_children | i32 | 否 |  |
| limit_children | i32 | 否 |  |
| max_tree_depth | i32 | 否 |  |
| url_id | String | 否 |  |
| user_id | String | 否 |  |
| anon_user_id | String | 否 |  |
| context_user_id | String | 否 |  |
| hash_tag | String | 否 |  |
| parent_id | String | 否 |  |
| direction | models::SortDirections | 否 |  |
| from_date | i64 | 否 |  |
| to_date | i64 | 否 |  |

## 响应

返回： [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## 示例

[inline-code-attrs-start title = 'get_comments 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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