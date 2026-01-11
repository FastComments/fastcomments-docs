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
async fn run_delete() -> Result<(), Error> {
    let params: DeleteFeedPostPublicParams = DeleteFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-2025-11-21-12345".to_string(),
        broadcast_id: Some("broadcast-7".to_string()),
        sso: Some("sso-token-abcdef123456".to_string()),
    };
    let response: DeleteFeedPostPublic200Response =
        delete_feed_post_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
