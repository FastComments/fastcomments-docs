## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_feed_post_params | models::CreateFeedPostParams | Yes |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Response

Returns: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_feed_post_public Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateFeedPostPublicParams = CreateFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_feed_post_params: models::CreateFeedPostParams {
            title: "Weekly Product Update".to_string(),
            body: "This week we shipped improvements to checkout and search.".to_string(),
            tags: vec!["product".to_string(), "release".to_string()],
            visibility: Some("public".to_string()),
            ..Default::default()
        },
        broadcast_id: Some("broadcast-2026-01".to_string()),
        sso: Some("sso-token-3f2e9b".to_string()),
    };
    let response: CreateFeedPostPublic200Response = create_feed_post_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
