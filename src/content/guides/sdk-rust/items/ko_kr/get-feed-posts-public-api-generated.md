req
tenantId
afterId

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| after_id | String | 아니오 |  |
| limit | i32 | 아니오 |  |
| tags | Vec<String> | 아니오 |  |
| sso | String | 아니오 |  |
| is_crawler | bool | 아니오 |  |
| include_user_info | bool | 아니오 |  |

## 응답

반환: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_feed_posts_response.rs)

## 예제

[inline-code-attrs-start title = 'get_feed_posts_public 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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