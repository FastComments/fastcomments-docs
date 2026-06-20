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

戻り値: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_response.rs)

## 例

[inline-code-attrs-start title = 'get_feed_posts の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetFeedPostsResponse, Error> {
    let cfg: &configuration::Configuration = &configuration;
    let params: GetFeedPostsParams = GetFeedPostsParams {
        tenant_id: String::from("acme-corp-tenant"),
        after_id: Some(String::from("post_987654321")),
        limit: Some(25),
        tags: Some(vec![String::from("product-updates"), String::from("release")]),
    };
    let response: GetFeedPostsResponse = get_feed_posts(cfg, params).await?;
    Ok(response)
}
[inline-code-end]

---