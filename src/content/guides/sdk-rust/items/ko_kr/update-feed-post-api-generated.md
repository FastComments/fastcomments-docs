## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| feed_post | models::FeedPost | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'update_feed_post 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_post_example() -> Result<(), Error> {
    let params: UpdateFeedPostParams = UpdateFeedPostParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/rocket-launch-2026".to_string(),
        feed_post: models::FeedPost {
            title: Some("Rocket Launch Successful".to_string()),
            body: Some("Acme Rockets launched the Atlas X at 10:00 UTC with no anomalies.".to_string()),
            author: Some("Acme Newsroom".to_string()),
            media: Some(vec![
                models::FeedPostMediaItem {
                    url: Some("https://cdn.acme.com/images/launch.jpg".to_string()),
                    caption: Some("Moments before liftoff".to_string()),
                    asset: None
                }
            ]),
            links: Some(vec![
                models::FeedPostLink {
                    title: Some("Detailed Coverage".to_string()),
                    url: Some("https://news.acme.com/coverage/atlas-x-launch".to_string())
                }
            ]),
            published: Some(true)
        }
    };
    let response: FlagCommentPublic200Response = update_feed_post(&configuration, params).await?;
    Ok(())
}
[inline-code-end]