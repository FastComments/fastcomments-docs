## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenant_id | String | 예 |  |
| post_id | String | 예 |  |
| update_feed_post_params | models::UpdateFeedPostParams | 예 |  |
| broadcast_id | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_response.rs)

## 예제

[inline-code-attrs-start title = 'update_feed_post_public 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = UpdateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".into(),
    post_id: "news/article-123".into(),
    update_feed_post_params: models::UpdateFeedPostParams {
        title: Some("Updated Headline".into()),
        content: Some("Revised content of the article with latest information.".into()),
        ..Default::default()
    },
    broadcast_id: Some("broadcast-001".into()),
    sso: Some("sso-token-abc123".into()),
};

let response: CreateFeedPostResponse = update_feed_post_public(&configuration, params).await?;
[inline-code-end]

---