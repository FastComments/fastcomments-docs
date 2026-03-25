## 매개변수

| 이름 | 타입 | 필수 | 설명 |
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

## 응답

반환: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_comments 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_comments() -> Result<(), Error> {
    let params: GetCommentsParams = GetCommentsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1),
        limit: Some(25),
        skip: Some(0),
        as_tree: Some(true),
        skip_children: Some(0),
        limit_children: Some(5),
        max_tree_depth: Some(3),
        url_id: Some("news/article/technology/ai-ethics".to_string()),
        user_id: Some("user_98765".to_string()),
        anon_user_id: Some("anon_abc123".to_string()),
        context_user_id: Some("moderator_12".to_string()),
        hash_tag: Some("aiethics".to_string()),
        parent_id: Some("comment_456".to_string()),
        direction: None,
    };
    let comments: GetComments200Response = get_comments(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---