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
async fn run_delete_comment() -> Result<DeleteCommentPublic200Response, Error> {
    let params: DeleteCommentPublicParams = DeleteCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("cmt-987654321"),
        broadcast_id: String::from("news/article/2025/11/tech-debate"),
        edit_key: Some(String::from("edk_ab12cd34")),
        sso: Some(String::from("sso_user_12345")),
    };
    let response: DeleteCommentPublic200Response = delete_comment_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
