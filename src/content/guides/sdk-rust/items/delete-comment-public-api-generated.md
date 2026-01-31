## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Response

Returns: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_comment_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteCommentPublicParams = DeleteCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-8a7b6c4d".to_string(),
        broadcast_id: "news/article/rocket-launch-2026".to_string(),
        edit_key: Some("editkey-9f8e7d6c".to_string()),
        sso: Some("sso-token-john.doe@acme.com".to_string()),
    };

    let response: DeleteCommentPublic200Response =
        delete_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
