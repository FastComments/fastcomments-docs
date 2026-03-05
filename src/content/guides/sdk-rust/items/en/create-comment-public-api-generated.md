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
let params: CreateCommentPublicParams = CreateCommentPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    url_id: "news/article/2026/fastcomments-launch".to_string(),
    broadcast_id: "live-2026-01-12".to_string(),
    comment_data: models::CommentData {
        content: "Excited about the launch — congrats to the team!".to_string(),
        user_display_name: Some("Jane Doe".to_string()),
        attachments: Vec::new(),
        mentions: Vec::new(),
    },
    session_id: Some("sess_01HXYZabc123".to_string()),
    sso: Some("sso-token-987654".to_string()),
};
let response: CreateCommentPublic200Response = create_comment_public(&configuration, params).await?;
[inline-code-end]
