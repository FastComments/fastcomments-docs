요청
tenantId
afterId

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| after_id | String | 아니요 |  |
| limit | i32 | 아니요 |  |
| tags | Vec<String> | 아니요 |  |
| sso | String | 아니요 |  |
| is_crawler | bool | 아니요 |  |
| include_user_info | bool | 아니요 |  |

## 응답

반환: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_feed_posts_public 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---