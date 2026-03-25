req
tenantId
afterId

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| after_id | String | いいえ |  |
| limit | i32 | いいえ |  |
| tags | Vec<String> | いいえ |  |

## レスポンス

返却: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_200_response.rs)

## 例

[inline-code-attrs-start title = 'get_feed_posts の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetFeedPostsParams = GetFeedPostsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("post_98765".to_string()),
        limit: Some(20),
        tags: Some(vec!["news".to_string(), "technology".to_string()]),
    };
    let feed: GetFeedPosts200Response = get_feed_posts(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---