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
async fn run() -> Result<DeleteCommentPublic200Response, Error> {
    let params: DeleteCommentPublicParams = DeleteCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("cmt-12345"),
        broadcast_id: String::from("news/article/2026-01-12"),
        edit_key: Some(String::from("editkey-abc123")),
        sso: Some(String::from("ssotoken-xyz789")),
    };
    let deleted: DeleteCommentPublic200Response = delete_comment_public(configuration, params).await?;
    Ok(deleted)
}
[inline-code-end]
