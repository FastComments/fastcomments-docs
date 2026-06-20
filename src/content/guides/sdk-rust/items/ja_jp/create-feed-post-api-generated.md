---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| create_feed_post_params | models::CreateFeedPostParams | はい |  |
| broadcast_id | String | いいえ |  |
| is_live | bool | いいえ |  |
| do_spam_check | bool | いいえ |  |
| skip_dup_check | bool | いいえ |  |

## レスポンス

戻り値: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_posts_response.rs)

## 例

[inline-code-attrs-start title = 'create_feed_post の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<CreateFeedPostsResponse, Error> {
    let create_feed: models::CreateFeedPostParams = models::CreateFeedPostParams {
        title: "Acme Product Launch".to_string(),
        body: "Acme Corp today launched the next-generation WidgetPro, offering improved performance and battery life.".to_string(),
        ..Default::default()
    };
    let params: CreateFeedPostParams = CreateFeedPostParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_feed_post_params: create_feed,
        broadcast_id: Some("launch-broadcast-2026".to_string()),
        is_live: Some(true),
        do_spam_check: Some(true),
        skip_dup_check: Some(false),
    };
    let response: CreateFeedPostsResponse = create_feed_post(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---