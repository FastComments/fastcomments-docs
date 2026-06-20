req
tenantId
afterId

## 参数

| Name | Type | 必需 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| after_id | String | 否 |  |
| limit | i32 | 否 |  |
| tags | Vec<String> | 否 |  |
| sso | String | 否 |  |
| is_crawler | bool | 否 |  |
| include_user_info | bool | 否 |  |

## 响应

返回: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_feed_posts_response.rs)

## 示例

[inline-code-attrs-start title = 'get_feed_posts_public 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example() -> Result<(), Error> {
    let params: GetFeedPostsPublicParams = GetFeedPostsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post_9f8d7c".to_string()),
        limit: Some(20),
        tags: Some(vec!["news".to_string(), "product-updates".to_string()]),
        sso: Some("sso-token-9a8b7c".to_string()),
        is_crawler: Some(false),
        include_user_info: Some(true),
    };
    let response: PublicFeedPostsResponse = get_feed_posts_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]