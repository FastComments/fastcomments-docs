## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| feed_post | models::FeedPost | 是 |  |

## 响应

返回：[`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'update_feed_post 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let feed_post = models::FeedPost {
        title: "Acme Corp Quarterly Update".to_string(),
        content: Some("Q2 results exceeded expectations with a 15% revenue growth.".to_string()),
        media: Some(vec![
            models::FeedPostMediaItem {
                asset: models::FeedPostMediaItemAsset {
                    url: "https://cdn.acme.com/media/q2-report.png".to_string(),
                    mime_type: Some("image/png".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
        ]),
        link: Some(models::FeedPostLink {
            url: "https://www.acme.com/reports/q2".to_string(),
            title: Some("Full Report".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let params = UpdateFeedPostParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/q2-update".to_string(),
        feed_post,
    };

    update_feed_post(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---