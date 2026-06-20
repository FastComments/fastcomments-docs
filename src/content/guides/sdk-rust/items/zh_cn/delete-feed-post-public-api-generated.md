## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| post_id | String | 是 |  |
| broadcast_id | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_feed_post_public_response.rs)

## 示例

[inline-code-attrs-start title = 'delete_feed_post_public 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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