## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Response

Returns: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_text_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_comment_text Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetCommentTextParams = GetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/2026/01/12/rust-async-example#cmt-42".to_string(),
        edit_key: Some("edk_3f2b7a9c".to_string()),
        sso: Some("sso:user:7890:token".to_string()),
    };
    let response: GetCommentText200Response = get_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
