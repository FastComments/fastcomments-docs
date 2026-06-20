## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| create_feed_post_params | models::CreateFeedPostParams | 是 |  |
| broadcast_id | String | 否 |  |
| is_live | bool | 否 |  |
| do_spam_check | bool | 否 |  |
| skip_dup_check | bool | 否 |  |

## 响应

返回：[`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_posts_response.rs)

## 示例

[inline-code-attrs-start title = 'create_feed_post 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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