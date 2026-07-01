req
tenantId
afterId

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| after_id | String | לא |  |
| limit | i32 | לא |  |
| tags | Vec<String> | לא |  |
| sso | String | לא |  |
| is_crawler | bool | לא |  |
| include_user_info | bool | לא |  |

## תגובה

מחזיר: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_feed_posts_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_feed_posts_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetFeedPostsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post123".to_string()),
        limit: Some(20),
        tags: Some(vec!["news".to_string(), "article".to_string()]),
        sso: Some("sso-token-xyz".to_string()),
        is_crawler: Some(false),
        include_user_info: Some(true),
    };
    let _response = get_feed_posts_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]