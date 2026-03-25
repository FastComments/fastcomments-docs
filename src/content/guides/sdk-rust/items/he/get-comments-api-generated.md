## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| page | i32 | לא |  |
| limit | i32 | לא |  |
| skip | i32 | לא |  |
| as_tree | bool | לא |  |
| skip_children | i32 | לא |  |
| limit_children | i32 | לא |  |
| max_tree_depth | i32 | לא |  |
| url_id | String | לא |  |
| user_id | String | לא |  |
| anon_user_id | String | לא |  |
| context_user_id | String | לא |  |
| hash_tag | String | לא |  |
| parent_id | String | לא |  |
| direction | models::SortDirections | לא |  |

## תגובה

מחזיר: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_comments'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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