## Parameters

| Name | Type | Required | Description |
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

## Response

Returns: [`GetComments200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_comments Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
        url_id: Some("news/article/2026/01/13".to_string()),
        user_id: Some("user-1234".to_string()),
        anon_user_id: Some("anon-5678".to_string()),
        context_user_id: Some("ctx-4321".to_string()),
        hash_tag: Some("breaking".to_string()),
        parent_id: Some("comment-9876".to_string()),
        direction: Some(models::SortDirections::Desc),
    };
    let response: GetComments200Response = get_comments(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
