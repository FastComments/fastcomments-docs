req
tenantId
afterId

## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| after_id | String | いいえ |  |
| limit | i32 | いいえ |  |
| tags | Vec<String> | いいえ |  |
| sso | String | いいえ |  |
| is_crawler | bool | いいえ |  |
| include_user_info | bool | いいえ |  |

## レスポンス

返却: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

## 例

[inline-code-attrs-start title = 'get_feed_posts_public の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<GetFeedPostsPublic200Response, Error> {
    let params: GetFeedPostsPublicParams = GetFeedPostsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post_987654".to_string()),
        limit: Some(25),
        tags: Some(vec!["news".to_string(), "technology".to_string()]),
        sso: Some("sso-token-abc123".to_string()),
        is_crawler: Some(false),
        include_user_info: Some(true),
    };
    let response: GetFeedPostsPublic200Response = get_feed_posts_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]