## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_comment() -> Result<GetComment200Response, Error> {
    let params: GetCommentParams = GetCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/2025/11/21/comment-7890".to_string(),
        include_replies: Some(true),
    };
    let comment: GetComment200Response = get_comment(configuration, params).await?;
    Ok(comment)
}
[inline-code-end]
