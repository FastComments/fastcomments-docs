## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| comment_data | models::CommentData | Yes |  |
| session_id | String | No |  |
| sso | String | No |  |

## Response

Returns: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_comment_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn post_comment() -> Result<CreateCommentPublic200Response, Error> {
    let params: CreateCommentPublicParams = CreateCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/2025/11/fast-rust-adoption"),
        broadcast_id: String::from("main-stream"),
        comment_data: models::CommentData {
            body: String::from("Great reporting — I especially liked the data on adoption rates."),
        },
        session_id: Some(String::from("sess_9f8e7d6c")),
        sso: Some(String::from("sso_token_abc123")),
    };
    let response: CreateCommentPublic200Response = create_comment_public(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
