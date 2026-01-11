## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| post_id | String | Yes |  |
| react_body_params | models::ReactBodyParams | Yes |  |
| is_undo | bool | No |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Response

Returns: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'react_feed_post_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let react_body_params: models::ReactBodyParams = models::ReactBodyParams {
    reaction: "like".to_string(),
};
let params: ReactFeedPostPublicParams = ReactFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    post_id: "news/article-2025-11-21".to_string(),
    react_body_params,
    is_undo: Some(false),
    broadcast_id: Some("broadcast-789".to_string()),
    sso: Some("sso-token-abc123".to_string()),
};
let response: ReactFeedPostPublic200Response = react_feed_post_public(&configuration, params).await?;
[inline-code-end]
