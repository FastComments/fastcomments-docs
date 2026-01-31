## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| post_id | String | Yes |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Response

Returns: [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_feed_post_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_feed_post_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn delete_post() -> Result<(), Error> {
    let params: DeleteFeedPostPublicParams = DeleteFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-2026-01-13-42".to_string(),
        broadcast_id: Some("broadcast-9876".to_string()),
        sso: Some("sso-token-abc123".to_string()),
    };
    let _response: DeleteFeedPostPublic200Response = delete_feed_post_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
