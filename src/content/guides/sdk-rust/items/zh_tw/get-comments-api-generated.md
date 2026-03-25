## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
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

## 回應

回傳: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_comments 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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