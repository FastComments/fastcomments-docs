## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| post_id | String | כן |  |
| broadcast_id | String | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_feed_post_public_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-delete_feed_post_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<DeleteFeedPostPublicResponse, Error> {
    let params: DeleteFeedPostPublicParams = DeleteFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-2026-06-19".to_string(),
        broadcast_id: Some("broadcast-789".to_string()),
        sso: Some("sso-token-abc123".to_string()),
    };
    let response: DeleteFeedPostPublicResponse = delete_feed_post_public(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---